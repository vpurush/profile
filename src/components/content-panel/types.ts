import { Document } from "@contentful/rich-text-types";
import { ContentfulPanel, PanelProps, PanelType } from "../panel/types";

export type ContentfulContentPanel = ContentfulPanel<{
  title: string;
  content: {
    json: Document;
  };
}>;

export type ContentPanelProps = PanelProps<{
  title: string;
  content: Document;
}>;
