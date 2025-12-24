import { createSignal, Show, type ParentProps } from "solid-js";

function DefaultLayout(props: ParentProps) {
  const [sidebarDisplay, setSidebarDisplay] = createSignal<boolean>(false);

  return (
    <div class="flex flex-col min-h-screen">
      <header class="bg-blue-500 p-4 flex items-center justify-between">
        <span class="text-white font-semibold">Header</span>
        {/* Small Screen Toggle */}
        <button
          class="md:hidden text-white cursor-pointer"
          onClick={() => setSidebarDisplay(true)}
        >
          Sidebar
        </button>
      </header>
      <div class="flex grow relative">
        {/* Desktop Sidebar */}
        <aside class="hidden md:flex md:w-80 p-4 bg-gray-200 order-1">
          <div class="bg-yellow-300 flex flex-col grow">Left Sidebar</div>
        </aside>

        {/* Mobile Sidebar Context */}
        <Show when={sidebarDisplay()}>
          {/* Overlay */}
          <div
            class="fixed inset-0 bg-black/50 z-40 md:hidden cursor-pointer"
            onClick={() => setSidebarDisplay(false)}
          />
          {/* Mobile Sidebar */}
          <aside class="fixed top-0 left-0 inset-y-0 w-80 bg-gray-200 p-4 z-50 md:hidden animate-slide-in flex flex-col">
            <div class="flex items-center justify-end mb-4">
              <button
                class="text-sm text-gray-600 cursor-pointer"
                onClick={() => setSidebarDisplay(false)}
              >
                ✕
              </button>
            </div>
            <div class="bg-yellow-300 flex-1 overflow-y-auto">Left Sidebar</div>
          </aside>
        </Show>

        <main class="grow p-4 order-2 md:order-1 flex flex-col">
          {props.children}
        </main>
      </div>
      <footer class="bg-gray-300 p-4">Footer</footer>
    </div>
  );
}

export default DefaultLayout;
