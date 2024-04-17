import { gql } from "@apollo/client/core";
import { getContentfulGraphqlClient } from "../../utils/contentful/get-contentful-graphql-client";

const query = gql`
  query {
    pageCollection (preview: true) {
      items {
        title
        slug
      }
    }
  }
`;

type ContentfulPage = {
  title: string;
  slug: string;
}

export const getPageCollection = () : Promise<ContentfulPage[]> => {
  return getContentfulGraphqlClient().query({ query }).then((value) => value.data.pageCollection.items);
}