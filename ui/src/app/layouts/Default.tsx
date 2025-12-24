import { createSignal, Show, type ParentProps } from "solid-js";
import { HiOutlineBars3, HiOutlineXMark } from "solid-icons/hi";

/// Provides a messenger layout.
function DefaultLayout(props: ParentProps) {
  const [sidebarDisplay, setSidebarDisplay] = createSignal<boolean>(false);

  return (
    // Main Container
    <div class="flex flex-col min-h-screen">
      {/* Header */}
      <header class="bg-blue-500 p-4 flex items-center justify-between">
        <span class="text-white font-semibold">Header</span>
        {/* Small Screen Toggle */}
        <button
          class="md:hidden text-white cursor-pointer"
          onClick={() => setSidebarDisplay(true)}
        >
          <HiOutlineBars3 class="size-6" />
        </button>
      </header>
      <div class="flex grow relative">
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
        <aside
          class={`
            fixed inset-y-0 left-0 w-80 bg-gray-200 p-4 z-50 md:hidden
            flex flex-col
            transform transition-transform duration-300 ease-out
            ${sidebarDisplay() ? "translate-x-0 pointer-events-auto shadow-xl" : "-translate-x-full pointer-events-none"}
          `}
        >
          <button
            class="mb-4 text-sm text-gray-600 self-end cursor-pointer"
            onClick={() => setSidebarDisplay(false)}
          >
            <HiOutlineXMark class="size-6" />
          </button>

          <div class="bg-gray-200 flex-1 overflow-y-auto overscroll-contain">
            Left Sidebar
          </div>
        </aside>

        <main class="grow p-4 order-2 md:order-1 flex flex-col">
          {props.children}
        </main>
      </div>
      <footer class="bg-gray-300 p-4">Footer</footer>
    </div>
  );
}

export default DefaultLayout;
