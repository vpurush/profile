import { Title } from "@solidjs/meta";
import Panel from "../panel/panel-component";
import { PageProps } from "./types";

export default function Page(props: PageProps) {
  console.log("pageprops", props.panels);
  return (
    <main>
      <Title>{props.title}</Title>
      <h1>{props.title}</h1>
      {props.panels.map((panel) => (
        <Panel {...panel} />
      ))}
    </main>
  );
}
