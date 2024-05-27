import { assertEquals } from "https://deno.land/std@0.224.0/assert/mod.ts";
import { validateYaml } from "./validateYaml.ts";

import blogPostSchema from "./fixtures/updateYaml/blogpost.schema.json" with {
  type: "json",
};

const blogPost = `title: "New Blog Post"
content: "This is the content of the blog post..."
publishedDate: "2023-08-25T15:00:00Z"
author:
  username: "authoruser"
  email: "author@example.com"
tags:
 - "Technology"
 - "Programming"`;

Deno.test("YAML parse pass", async () => {
  const data = await validateYaml(blogPost, blogPostSchema);

  assertEquals(data, {
    title: "New Blog Post",
    content: "This is the content of the blog post...",
    publishedDate: "2023-08-25T15:00:00Z",
    author: {
      username: "authoruser",
      email: "author@example.com",
    },
    tags: ["Technology", "Programming"],
  });
});

Deno.test("YAML parse fail", async () => {
  // first line removed to make the YAML invalid
  const data = blogPost.split("\n").slice(1).join("\n");
  await validateYaml(
    data,
    blogPostSchema,
  ).catch((err) => {
    assertEquals(err[0].message, "must have required property 'title'");
  });
});
