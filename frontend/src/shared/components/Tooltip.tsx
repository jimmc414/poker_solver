import React, { useState, useRef, useCallback } from 'react';
import { theme } from '../styles/theme';

interface TooltipProps {
  content: React.ReactNode;
  children: React.ReactNode;
  delay?: number;
}

/**
 * Hover tooltip that appears after a configurable delay.
 * Positions itself above the target element.
 */
export const Tooltip: React.FC<TooltipProps> = ({ content, children, delay = 500 }) => {
  const [visible, setVisible] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const showTooltip = useCallback(() => {
    timerRef.current = setTimeout(() => {
      setVisible(true);
    }, delay);
  }, [delay]);

  const hideTooltip = useCallback(() => {
    if (timerRef.current !== null) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }
    setVisible(false);
  }, []);

  const containerStyle: React.CSSProperties = {
    position: 'relative',
    display: 'inline-block',
  };

  const tooltipStyle: React.CSSProperties = {
    position: 'absolute',
    bottom: '100%',
    left: '50%',
    transform: 'translateX(-50%)',
    marginBottom: 6,
    padding: `${theme.spacing.xs}px ${theme.spacing.sm}px`,
    backgroundColor: theme.colors.bgTertiary,
    color: theme.colors.text,
    border: `1px solid ${theme.colors.borderLight}`,
    borderRadius: theme.borderRadius.sm,
    fontSize: theme.fontSize.sm,
    whiteSpace: 'nowrap',
    pointerEvents: 'none',
    zIndex: 1000,
    boxShadow: '0 2px 8px rgba(0, 0, 0, 0.4)',
  };

  return (
    <span
      style={containerStyle}
      onMouseEnter={showTooltip}
      onMouseLeave={hideTooltip}
    >
      {children}
      {visible && (
        <span style={tooltipStyle} data-testid="tooltip" role="tooltip">
          {content}
        </span>
      )}
    </span>
  );
};
