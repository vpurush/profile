import { MetaProvider, Title } from "@solidjs/meta";
import { createAsync, Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { createResource, Suspense } from "solid-js";
import "./app.css";
import { getPageCollection } from "./components/page/get-pages-simply";

export const route = {
  load: () => {
    console.log("load called");
  },
};

export default function App() {
  const [pages] = createResource(async () => {
    const pageC = await getPageCollection();
    console.log("pageC", pageC);
    return pageC;
    // return Promise.resolve({});
  });

  // pages.

  return (
    <Router
      root={(props) => (
        <MetaProvider>
          <Suspense>
            <span>{pages()?.map(p => p.title).join(",")}</span>
          </Suspense>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
