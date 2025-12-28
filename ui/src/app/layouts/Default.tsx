import { createSignal, Show, type ParentProps } from "solid-js";

import Header from "@/app/layouts/elements/Header";
import MobileSidebar from "@/app/layouts/elements/MobileSidebar";

/// Provides a messenger layout.
function DefaultLayout(props: ParentProps) {
  const [sidebarDisplay, setSidebarDisplay] = createSignal<boolean>(false);

  return (
    // Main Container
    <div class="flex flex-col h-screen overflow-hidden">
      {/* Header */}
      <Header toggleSidebar={() => setSidebarDisplay(!sidebarDisplay())} />

      {/* Main Container */}
      <div class="flex flex-1 overflow-hidden">
        {/* Desktop Sidebar */}
        <aside class="hidden md:flex md:w-80 p-4 bg-gray-200 order-1 shrink-0">
          <div class="flex flex-col grow">Left Sidebar</div>
        </aside>

        {/* Mobile Sidebar Context */}
        <Show when={sidebarDisplay()}>
          {/* Overlay */}
          <div
            class="fixed inset-0 bg-black/50 z-40 md:hidden cursor-pointer"
            onClick={() => setSidebarDisplay(false)}
          />
        </Show>

        {/* Mobile Sidebar */}
        <MobileSidebar
          isOpen={sidebarDisplay()}
          onClose={() => setSidebarDisplay(false)}
        />

        <main class="flex-1 p-4 order-2 md:order-1 flex flex-col overflow-hidden">
          {props.children}
        </main>
      </div>
      {/*<footer class="bg-gray-300 p-4">Footer</footer>*/}
    </div>
  );
}

export default DefaultLayout;
