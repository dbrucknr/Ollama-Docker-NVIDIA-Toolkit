import { HiOutlineXMark } from "solid-icons/hi";

type MobileSidebarProps = {
  isOpen: boolean;
  onClose: () => void;
};

function MobileSidebar(props: MobileSidebarProps) {
  return (
    <aside
      class={`
        fixed inset-y-0 left-0 w-80 bg-gray-200 p-4 z-50 md:hidden
        flex flex-col
        transform transition-transform duration-300 ease-out
        ${props.isOpen ? "translate-x-0 pointer-events-auto shadow-xl" : "-translate-x-full pointer-events-none"}
      `}
    >
      <button
        class="mb-4 text-sm text-gray-600 self-end cursor-pointer"
        onClick={props.onClose}
      >
        <HiOutlineXMark class="size-6" />
      </button>

      <div class="bg-gray-200 flex-1 overflow-y-auto overscroll-contain">
        Left Sidebar
      </div>
    </aside>
  );
}

export default MobileSidebar;
