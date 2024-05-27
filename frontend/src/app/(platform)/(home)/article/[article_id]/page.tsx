import { get_id } from "@/lib/utils";

interface IParams {
  article_id: string;
}

const ArticlePage = ({ params }: { params: IParams }) => {
  return <div>{get_id(params.article_id)}</div>;
};

export default ArticlePage;
