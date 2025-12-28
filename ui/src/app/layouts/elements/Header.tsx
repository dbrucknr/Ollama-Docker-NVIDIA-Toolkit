import { HiOutlineBars3 } from "solid-icons/hi";

type HeaderProps = {
  toggleSidebar: () => void;
};

function Header(props: HeaderProps) {
  return (
    <header class="bg-blue-500 p-4 flex items-center justify-between">
      <span class="text-white font-semibold">Header</span>
      {/* Small Screen Toggle */}
      <button
        class="md:hidden text-white cursor-pointer"
        onClick={props.toggleSidebar}
      >
        <HiOutlineBars3 class="size-6" />
      </button>
    </header>
  );
}

export default Header;
