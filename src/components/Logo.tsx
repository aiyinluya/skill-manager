export function Logo({ size = 28 }: { size?: number }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 64 64"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden
    >
      <defs>
        <linearGradient id="lg-bg" x1="0" y1="0" x2="64" y2="64">
          <stop offset="0%" stopColor="#6366f1" />
          <stop offset="100%" stopColor="#a855f7" />
        </linearGradient>
        <linearGradient id="lg-card" x1="0" y1="0" x2="32" y2="40">
          <stop offset="0%" stopColor="#ffffff" />
          <stop offset="100%" stopColor="#dbe2ff" />
        </linearGradient>
      </defs>
      <rect x="2" y="2" width="60" height="60" rx="16" fill="url(#lg-bg)" />
      {/* 堆叠卡片 */}
      <rect
        x="16"
        y="34"
        width="26"
        height="18"
        rx="4"
        fill="#ffffff"
        opacity="0.55"
        transform="rotate(-8 29 43)"
      />
      <rect
        x="14"
        y="20"
        width="30"
        height="20"
        rx="5"
        fill="url(#lg-card)"
        transform="rotate(-8 29 30)"
      />
      <rect x="19" y="26" width="16" height="2.6" rx="1.3" fill="#6366f1" opacity="0.8" transform="rotate(-8 29 30)" />
      <rect x="19" y="31" width="11" height="2.6" rx="1.3" fill="#94a3b8" opacity="0.8" transform="rotate(-8 29 30)" />
      {/* 齿轮 */}
      <g transform="translate(42 40)">
        <circle r="9" fill="#0c0d11" opacity="0.85" />
        <circle r="3.4" fill="#a5b4fc" />
        {Array.from({ length: 8 }).map((_, i) => {
          const a = (i * Math.PI) / 4;
          const x = Math.cos(a) * 10.5;
          const y = Math.sin(a) * 10.5;
          return (
            <rect
              key={i}
              x={x - 1.4}
              y={y - 1.4}
              width="2.8"
              height="2.8"
              rx="0.8"
              fill="#a5b4fc"
              transform={`rotate(${(a * 180) / Math.PI} ${x} ${y})`}
            />
          );
        })}
      </g>
      {/* 星光 */}
      <path
        d="M46 14 l1.6 4.2 4.2 1.6 -4.2 1.6 -1.6 4.2 -1.6 -4.2 -4.2 -1.6 4.2 -1.6 z"
        fill="#fde68a"
      />
    </svg>
  );
}
