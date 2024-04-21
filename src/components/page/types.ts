import { ContentfulPanel, PanelProps } from "../panel/types";

export type ContentfulPage = {
  title: string;
  slug: string;
  panelsCollection: {
    items: ContentfulPanel<any>[];
  };
};

export type PageProps = {
  title: string;
  slug: string;
  panels: PanelProps<unknown>[];
};
