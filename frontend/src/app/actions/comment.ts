//
// export async function get_user_articles(
//   username: string,
// ): Promise<ResultPaging<FullArticle>> {
//   const url = new URLSearchParams();
//
//   CursorPaginationToUrlParams(url, cursor);
//
//   return fetch(
//     `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/users/${username}/articles?${url}`,
//     {
//       headers: {
//         Authorization: `Bearer ${sessionStorage.getItem("session")}`,
//       },
//     },
//   ).then(async (res) => {
//     if (!res.ok) {
//       throw new Error(`${res.status} - ${await res.text()}`);
//     }
//     return await res.json();
//   });
// }
