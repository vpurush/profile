import { Title } from "@solidjs/meta";
import { useParams } from "@solidjs/router";
import { createResource } from "solid-js";
import Counter from "~/components/Counter";
import { getPage } from "~/components/page/get-page";

export default function Home() {

  const [page] = createResource(async () => {
    const params = useParams();
    const pageC = await getPage(params.slug);
    return pageC;
  });

  return page() ? (
    <main>
      <Title>{page()?.title}</Title>
      <h2>{page()?.title}</h2>
      <p>{useParams().slug}</p>
    </main>
  ) : (
    <span> Not found </span>
  );
}
