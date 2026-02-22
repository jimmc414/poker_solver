import React, { useCallback, useMemo } from 'react';
import { theme } from '../shared/styles/theme';
import { useRangeState } from './hooks/useRangeState';
import { RangeGrid } from './components/RangeGrid';
import { PaintbrushTools } from './components/PaintbrushTools';
import { WeightControls } from './components/WeightControls';
import { SuitExpander } from './components/SuitExpander';
import { PresetSelector } from './components/PresetSelector';
import { ComboLock } from './components/ComboLock';
import { RangeSummary } from './components/RangeSummary';
import { RangeColorLegend } from './components/RangeColorLegend';

/**
 * Range Builder page root component.
 * Layout: RangeGrid (left), controls panel (right) containing
 * PaintbrushTools, WeightControls, PresetSelector, SuitExpander,
 * ComboLock, RangeSummary, and RangeColorLegend.
 */
export const RangeBuilderPage: React.FC = () => {
  const {
    state,
    dispatch,
    setPaintMode,
    setPaintWeight,
    loadPreset,
    toggleLock,
  } = useRangeState();

  const selectedIndex = useMemo(() => {
    const selectedArray = Array.from(state.selectedCells);
    return selectedArray.length === 1 ? selectedArray[0] : null;
  }, [state.selectedCells]);

  const hasLockedSelected = useMemo(() => {
    return Array.from(state.selectedCells).some((idx) => state.locked[idx]);
  }, [state.selectedCells, state.locked]);

  const handleToggleLock = useCallback(() => {
    for (const idx of state.selectedCells) {
      toggleLock(idx);
    }
  }, [state.selectedCells, toggleLock]);

  return (
    <div
      data-testid="range-builder-page"
      style={{
        display: 'flex',
        gap: theme.spacing.lg,
        padding: theme.spacing.lg,
        height: '100%',
        overflow: 'auto',
      }}
    >
      {/* Left: Hand Matrix Grid */}
      <div style={{ flexShrink: 0 }}>
        <RangeGrid state={state} dispatch={dispatch} />
      </div>

      {/* Right: Controls Panel */}
      <div
        style={{
          display: 'flex',
          flexDirection: 'column',
          gap: theme.spacing.md,
          minWidth: 240,
          maxWidth: 320,
        }}
      >
        <PaintbrushTools
          activeMode={state.paintMode}
          onModeChange={setPaintMode}
        />

        <WeightControls
          weight={state.paintWeight}
          onWeightChange={setPaintWeight}
        />

        <PresetSelector
          activePreset={state.activePreset}
          onLoadPreset={loadPreset}
        />

        <SuitExpander selectedIndex={selectedIndex} />

        <ComboLock
          selectedCount={state.selectedCells.size}
          hasLockedSelected={hasLockedSelected}
          onToggleLock={handleToggleLock}
        />

        <RangeSummary weights={state.weights} />

        <RangeColorLegend />
      </div>
    </div>
  );
};
