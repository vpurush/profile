import { contentPanelDecorator } from "../content-panel/content-panel-decorator";
import { ContentfulContentPanel } from "../content-panel/types";
import { ContentfulPanel, PanelProps, PanelType } from "./types";

const isContentPanel = (
  panel: ContentfulPanel<unknown>,
): panel is ContentfulContentPanel => {
  return panel.__typename === PanelType.ContentPanel;
};

export const panelDecorator = <T extends ContentfulContentPanel>(
  panel: ContentfulPanel<T>,
): PanelProps<{}> => {
  const panelType = panel.__typename;

  console.log("panelType", panelType)

  if (isContentPanel(panel)) {
    console.log("ocntent panel")
    return {
      panelType,
      ...contentPanelDecorator(panel),
    };
  }

  return {
    panelType,
  };
};
