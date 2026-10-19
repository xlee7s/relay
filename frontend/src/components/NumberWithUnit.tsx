import { InputNumber } from 'antd';
import type { InputNumberProps } from 'antd';
import type { CSSProperties, ReactNode } from 'react';

export interface NumberWithUnitProps extends InputNumberProps {
  prefixLabel?: ReactNode;
  unit?: ReactNode;
  style?: CSSProperties;
}

/** A connected numeric field with optional prefix and unit cells. */
export default function NumberWithUnit({
  prefixLabel,
  unit,
  style,
  className,
  ...inputProps
}: NumberWithUnitProps) {
  const classes = [
    'rp-number-combo',
    prefixLabel != null && 'rp-number-combo--prefixed',
    unit != null && 'rp-number-combo--suffixed',
    className,
  ].filter(Boolean).join(' ');

  return (
    <div className={classes} style={style}>
      {prefixLabel != null && <span className="rp-number-combo__addon rp-number-combo__addon--prefix">{prefixLabel}</span>}
      <InputNumber {...inputProps} className="rp-number-combo__input" />
      {unit != null && <span className="rp-number-combo__addon rp-number-combo__addon--unit">{unit}</span>}
    </div>
  );
}
