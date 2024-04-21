import { Title } from "@solidjs/meta";
import ContentPanel from "../content-panel/content-panel-component";
import { PanelProps, PanelType } from "./types";

export default function Panel<T>(props: PanelProps<any>) {
  if (props.panelType === PanelType.ContentPanel) {
    return <ContentPanel {...props} />;
  }
  return undefined;
}
