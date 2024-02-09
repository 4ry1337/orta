import PostgresAdapter from '@auth/pg-adapter';
import NextAuth, { AuthOptions } from 'next-auth';
import { Adapter } from 'next-auth/adapters';
import CredentialsProvider from 'next-auth/providers/credentials';
import GitHubProvider from 'next-auth/providers/github';
import GoogleProvider from 'next-auth/providers/google';
import { Pool } from 'pg';

const pool = new Pool({
  host: process.env.POSTGRES_HOST as string,
  port: 5432,
  user: process.env.POSTGRES_USER as string,
  database: 'dev',
  password: process.env.POSTGRES_PASSWORD as string,
});

const authOptions: AuthOptions = {
  providers: [
    GoogleProvider({
      clientId: process.env.GOOGLE_CLIENT_ID as string,
      clientSecret: process.env
        .GOOGLE_CLIENT_SECRET as string,
    }),
    GitHubProvider({
      clientId: process.env.GITHUB_ID as string,
      clientSecret: process.env.GITHUB_SECRET as string,
    }),
    CredentialsProvider({
      name: 'credentials',
      credentials: {
        email: { label: 'email', type: 'email' },
        password: { label: 'password', type: 'password' },
      },
      async authorize(credentials) {
        const { email, password } = credentials as any;
        const res = await fetch(
          'http://localhost:5000/api/auth/signin',
          {
            method: 'POST',
            body: JSON.stringify({
              email,
              password,
            }),
            headers: {
              'Content-Type': 'application/json',
            },
          }
        );
        const user = await res.json();
        if (res.ok && user) {
          console.log(user);
          return user;
        }
        return null;
      },
    }),
  ],
  adapter: PostgresAdapter(pool) as Adapter,
  debug: true,
  session: {
    strategy: 'jwt',
  },
};

// user: {
//   id: '52571621',
//   name: 'yskak.rakhat',
//   email: 'yskak.rakhat@gmail.com',
//   image: 'https://avatars.githubusercontent.com/u/52571621?v=4'
// }
// account: {
//   provider: 'github',
//   type: 'oauth',
//   providerAccountId: '52571621',
//   access_token: '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',
//   token_type: 'bearer',
//   scope: 'read:user,user:email'
// }
// profile: {
//   login: '4ry1337',
//   id: 52571621,
//   node_id: 'MDQ6VXNlcjUyNTcxNjIx',
//   avatar_url: 'https://avatars.githubusercontent.com/u/52571621?v=4',
//   gravatar_id: '',
//   url: 'https://api.github.com/users/4ry1337',
//   html_url: 'https://github.com/4ry1337',
//   followers_url: 'https://api.github.com/users/4ry1337/followers',
//   following_url: 'https://api.github.com/users/4ry1337/following{/other_user}',
//   gists_url: 'https://api.github.com/users/4ry1337/gists{/gist_id}',
//   starred_url: 'https://api.github.com/users/4ry1337/starred{/owner}{/repo}',
//   subscriptions_url: 'https://api.github.com/users/4ry1337/subscriptions',
//   organizations_url: 'https://api.github.com/users/4ry1337/orgs',
//   repos_url: 'https://api.github.com/users/4ry1337/repos',
//   events_url: 'https://api.github.com/users/4ry1337/events{/privacy}',
//   received_events_url: 'https://api.github.com/users/4ry1337/received_events',
//   type: 'User',
//   site_admin: false,
//   name: 'yskak.rakhat',
//   company: null,
//   blog: 'https://www.instagram.com/yskak.rakhat/',
//   location: 'Astana, Kazakhstan',
//   email: 'yskak.rakhat@gmail.com',
//   hireable: true,
//   bio: 'Software Engineer from Kazakhstan',
//   twitter_username: null,
//   public_repos: 21,
//   public_gists: 0,
//   followers: 2,
//   following: 9,
//   created_at: '2019-07-05T12:41:04Z',
//   updated_at: '2024-01-26T13:44:15Z',
//   private_gists: 0,
//   total_private_repos: 2,
//   owned_private_repos: 2,
//   disk_usage: 320241,
//   collaborators: 2,
//   two_factor_authentication: false,
//   plan: {
//     name: 'free',
//     space: 976562499,
//     collaborators: 0,
//     private_repos: 10000
//   }
// }
// token: {
//   name: 'yskak.rakhat',
//   email: 'yskak.rakhat@gmail.com',
//   picture: 'https://avatars.githubusercontent.com/u/52571621?v=4',
//   sub: '52571621'
// }

const handler = NextAuth(authOptions);

export { handler as GET, handler as POST };
