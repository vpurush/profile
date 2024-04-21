import { useParams } from "@solidjs/router";
import { createResource } from "solid-js";
import { getPage } from "~/components/page/get-page";
import Page from "~/components/page/page-component";
import { pageDecorator } from "~/components/page/page-decorator";

export default function SlugRenderer() {
  const [page] = createResource(async () => {
    const params = useParams();
    const contentfulPage = await getPage(params.slug);
    return contentfulPage ? pageDecorator(contentfulPage) : undefined;
  });

  return page() ? (
    <Page {...page()!} />
  ) : (
    <span> Not found </span>
  );
}
