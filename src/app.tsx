import { MetaProvider } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { createResource, Suspense } from "solid-js";
import "./app.css";
import { getPageCollection } from "./components/page/get-pages";


export default function App() {
  // const [pages] = createResource(async () => {
  //   const pageC = await getPageCollection();
  //   return pageC;
  // });


  return (
    <Router
      root={(props) => (
        <MetaProvider>
          <Suspense>
            {/* <span>{pages()?.map(p => p.title).join(",")}</span> */}
            {props.children}
          </Suspense>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
