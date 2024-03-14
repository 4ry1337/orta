interface IParams {
  user_id: string;
}

const UserPage = ({ params }: { params: IParams }) => {
  return <div>{params.user_id}</div>;
};

export default UserPage;
