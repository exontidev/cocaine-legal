interface DimmerProps {
  dimmed: boolean;
  className?: string;
}

export default function Dimmer({ dimmed, className }: DimmerProps) {
  return (
    <div
      className={
        `fixed inset-0 w-screen h-screen bg-black transition-opacity duration-200 ${
          dimmed
            ? "opacity-30 pointer-events-none"
            : "opacity-0 pointer-events-none"
        } ` + className
      }
    ></div>
  );
}
