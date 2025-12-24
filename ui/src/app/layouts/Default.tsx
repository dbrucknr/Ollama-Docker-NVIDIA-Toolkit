import type { ParentComponent, ParentProps } from "solid-js";

export const DefaultLayout: ParentComponent = (props: ParentProps) => {
  return (
    <div class="flex flex-col min-h-screen">
      <header class="bg-blue-500 p-4">Header</header>
      <main class="grow p-4">{props.children}</main>
      <footer class="bg-gray-300 p-4">Footer</footer>
    </div>
  );
};
