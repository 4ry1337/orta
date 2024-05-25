interface IParams {
  article_id: string;
}

const ArticlePage = ({ params }: { params: IParams }) => {
  return <div>{params.article_id}</div>;
};

export default ArticlePage;
