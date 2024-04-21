import { panelDecorator } from "../panel/panel-decorator";
import { ExtractBarePanel, ExtractBarePanelProps } from "../panel/types";
import { ContentfulContentPanel, ContentPanelProps } from "./types";

export const contentPanelDecorator = (
  contentfulContentPanel: ContentfulContentPanel,
): ExtractBarePanelProps<ContentPanelProps> => {
  return {
    title: contentfulContentPanel.title,
    content: contentfulContentPanel.content.json,
  };
};
