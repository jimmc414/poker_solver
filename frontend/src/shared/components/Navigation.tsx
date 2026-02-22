import React from 'react';
import type { AppMode } from '../types/common';
import { theme } from '../styles/theme';

interface NavEntry {
  mode: AppMode;
  label: string;
  shortcut?: string;
}

const NAV_ENTRIES: NavEntry[] = [
  { mode: 'study', label: 'Study', shortcut: 'J' },
  { mode: 'solve', label: 'Solve', shortcut: 'S' },
  { mode: 'practice', label: 'Practice', shortcut: 'P' },
  { mode: 'analyze', label: 'Analyze', shortcut: 'Q' },
  { mode: 'range-builder', label: 'Range Builder' },
  { mode: 'nodelock', label: 'Nodelock' },
  { mode: 'reports', label: 'Reports' },
  { mode: 'settings', label: 'Settings' },
];

interface NavigationProps {
  activeMode: AppMode;
  onModeChange: (mode: AppMode) => void;
  collapsed?: boolean;
}

/**
 * Sidebar navigation component.
 * Displays mode entries with keyboard shortcut hints.
 */
export const Navigation: React.FC<NavigationProps> = ({
  activeMode,
  onModeChange,
  collapsed = false,
}) => {
  const sidebarStyle: React.CSSProperties = {
    display: 'flex',
    flexDirection: 'column',
    width: collapsed ? 48 : 200,
    backgroundColor: theme.colors.bgSecondary,
    borderRight: `1px solid ${theme.colors.border}`,
    padding: `${theme.spacing.sm}px 0`,
    transition: 'width 0.2s ease',
    overflow: 'hidden',
    flexShrink: 0,
  };

  const headerStyle: React.CSSProperties = {
    padding: `${theme.spacing.md}px ${theme.spacing.md}px`,
    fontSize: theme.fontSize.lg,
    fontWeight: 700,
    color: theme.colors.accent,
    whiteSpace: 'nowrap',
    borderBottom: `1px solid ${theme.colors.border}`,
    marginBottom: theme.spacing.sm,
  };

  return (
    <nav style={sidebarStyle} data-testid="navigation">
      {!collapsed && <div style={headerStyle}>GTO Solver</div>}
      {NAV_ENTRIES.map((entry) => (
        <NavItem
          key={entry.mode}
          entry={entry}
          isActive={activeMode === entry.mode}
          collapsed={collapsed}
          onClick={() => onModeChange(entry.mode)}
        />
      ))}
    </nav>
  );
};

interface NavItemProps {
  entry: NavEntry;
  isActive: boolean;
  collapsed: boolean;
  onClick: () => void;
}

const NavItem: React.FC<NavItemProps> = ({ entry, isActive, collapsed, onClick }) => {
  const itemStyle: React.CSSProperties = {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: `${theme.spacing.sm}px ${theme.spacing.md}px`,
    cursor: 'pointer',
    backgroundColor: isActive ? theme.colors.bgTertiary : 'transparent',
    color: isActive ? theme.colors.text : theme.colors.textSecondary,
    borderLeft: isActive ? `3px solid ${theme.colors.accent}` : '3px solid transparent',
    fontSize: theme.fontSize.md,
    whiteSpace: 'nowrap',
    transition: 'background-color 0.15s ease',
    userSelect: 'none',
  };

  const shortcutStyle: React.CSSProperties = {
    fontSize: theme.fontSize.xs,
    color: theme.colors.textMuted,
    backgroundColor: theme.colors.bg,
    padding: '1px 5px',
    borderRadius: theme.borderRadius.sm,
    fontFamily: 'monospace',
  };

  return (
    <div
      style={itemStyle}
      onClick={onClick}
      data-testid={`nav-${entry.mode}`}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          onClick();
        }
      }}
    >
      {!collapsed && <span>{entry.label}</span>}
      {!collapsed && entry.shortcut && <span style={shortcutStyle}>{entry.shortcut}</span>}
    </div>
  );
};
