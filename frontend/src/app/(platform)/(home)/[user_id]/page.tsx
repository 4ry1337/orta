import { Suspense } from 'react';

interface IParams {
  user_id: string;
}

const getUser = async (user_id: string) => {
  const res = await fetch(
    `http://localhost:5000/api/users/${user_id}`
  );
  return res.json();
};

const UserPage = async ({
  params,
}: {
  params: IParams;
}) => {
  const user = await getUser(params.user_id);
  console.log(user);
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <div>{user.username}</div>
    </Suspense>
  );
};

export default UserPage;
