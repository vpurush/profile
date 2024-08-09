let pageQuery = "
query($slug: String!) {
  pageCollection (preview: true, where: {slug: $slug}) {
    items {
      title
      slug
      panelsCollection {
        items {
          __typename
        }
      }
    }
  }
}
";