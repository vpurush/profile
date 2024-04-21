export enum PanelType {
  ContentPanel = "ContentPanel",
}

export type ContentfulPanel<T> = {
  __typename: PanelType;
} & T;

export type PanelProps<T> = {
  panelType: PanelType;
} & T;

export type ExtractBarePanel<T> =
  T extends ContentfulPanel<infer R> ? R : never;

export type ExtractBarePanelProps<T> =
  T extends PanelProps<infer R> ? R : never;
