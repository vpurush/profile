import { ContentPanelProps } from "./types";
import { documentToHtmlString } from "@contentful/rich-text-html-renderer";

export default function ContentPanel(props: ContentPanelProps) {
  return (
    <main>
      <h2>{props.title}</h2>
      <p innerHTML={documentToHtmlString(props.content)}></p>
    </main>
  );
}
