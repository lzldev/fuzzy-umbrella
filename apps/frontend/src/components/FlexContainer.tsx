import type { ReactNode } from "react";

export function FlexContainer({ children }: { children: ReactNode }) {
  return (
    <div className="flex flex-col justify-center items-center min-h-screen h-full w-full overflow-y-auto">
      {children}
    </div>
  );
}
