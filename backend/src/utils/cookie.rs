//TODO: implement cookie chunks if data is too large
//
//
// Uncomment to recalculate the estimated size
// of an empty session cookie
// import { serialize } from "cookie"
// console.log(
//   "Cookie estimated to be ",
//   serialize(`__Secure.authjs.session-token.0`, "", {
//     expires: new Date(),
//     httpOnly: true,
//     maxAge: Number.MAX_SAFE_INTEGER,
//     path: "/",
//     sameSite: "strict",
//     secure: true,
//     domain: "example.com",
//   }).length,
//   " bytes"
// )
// const CHUNK_SIZE = ALLOWED_COOKIE_SIZE - ESTIMATED_EMPTY_COOKIE_SIZE
// /** Given a cookie, return a list of cookies, chunked to fit the allowed cookie size. */
//   #chunk(cookie: Cookie): Cookie[] {
//     const chunkCount = Math.ceil(cookie.value.length / CHUNK_SIZE)
//
//     if (chunkCount === 1) {
//       this.#chunks[cookie.name] = cookie.value
//       return [cookie]
//     }
//
//     const cookies: Cookie[] = []
//     for (let i = 0; i < chunkCount; i++) {
//       const name = `${cookie.name}.${i}`
//       const value = cookie.value.substr(i * CHUNK_SIZE, CHUNK_SIZE)
//       cookies.push({ ...cookie, name, value })
//       this.#chunks[name] = value
//     }
//
//     this.#logger.debug("CHUNKING_SESSION_COOKIE", {
//       message: `Session cookie exceeds allowed ${ALLOWED_COOKIE_SIZE} bytes.`,
//       emptyCookieSize: ESTIMATED_EMPTY_COOKIE_SIZE,
//       valueSize: cookie.value.length,
//       chunks: cookies.map((c) => c.value.length + ESTIMATED_EMPTY_COOKIE_SIZE),
//     })
//
//     return cookies
//   }

// const ALLOWED_COOKIE_SIZE: u16 = 4096;
// const ESTIMATED_EMPTY_COOKIE_SIZE: u16 = 160;
// const CHUNK_SIZE: u16 = ALLOWED_COOKIE_SIZE - ESTIMATED_EMPTY_COOKIE_SIZE;

// pub struct CookieOptions {
//     name: String,
//     http_only: bool,
//     same_site: SameSite,
//     max_age: Duration,
//     path: String,
// }

// pub fn chunk(value: String, options: CookieOptions) -> Vec<Cookie> {
//     //asumc that cookie will not be larger than u32
//     let length = value.len();
//     let length_u32: u32 = length.try_into().unwrap();
//     let chunk_count: i32 = (length_u32 as f32 / CHUNK_SIZE as f32).ceil() as i32;
//
//     let mut cookies = vec![];
//
//     if chunk_count == 1 {
//         cookies.push(
//             Cookie::build((options.name, value))
//                 .http_only(options.http_only)
//                 .path(options.path)
//                 .same_site(options.same_site)
//                 .max_age(options.max_age)
//                 .into(),
//         );
//         return cookies;
//     }
//
//     for i in 0..chunk_count {
//         let name = format!("{}.{}", options.name, i);
//         let start: usize = i as usize * CHUNK_SIZE as usize;
//         let end: usize = cmp::min(start + CHUNK_SIZE as usize, length);
//         let cookie_value = &value[start..end];
//         cookies.push(
//             Cookie::build((name, cookie_value))
//                 .http_only(options.http_only)
//                 .path(options.path)
//                 .same_site(options.same_site)
//                 .max_age(options.max_age)
//                 .into(),
//         );
//     }
//
//     cookies
// }
