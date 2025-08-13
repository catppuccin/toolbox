import { assertEquals } from "jsr:@std/assert@1.0.14";
import { validateYaml } from "./validateYaml.ts";

import blogPostSchema from "./fixtures/updateYaml/blogPost.schema.json" with {
  type: "json",
};
import blogPostsSchema from "./fixtures/updateYaml/blogPosts.schema.json" with {
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

const multipleBlogPosts = `- title: "New Blog Post"
  content: "This is the content of the blog post..."
  publishedDate: "2023-08-25T15:00:00Z"
  author:
    username: "hammy"
    email: "hammy@catppuccin.com"
  tags: ["Technology", "Programming"]
- title: "Another Blog Post"
  content: "This is the content of the blog post..."
  publishedDate: "2023-08-25T15:00:00Z"
  author:
    username: "pigeon"
    email: "pigeon@catppuccin.com"
  tags: ["Technology", "Programming"]`;

Deno.test("YAML parse multiple schemas pass", async () => {
  const data = await validateYaml(multipleBlogPosts, blogPostsSchema, {
    schemas: [blogPostSchema],
  });

  assertEquals(data, [
    {
      title: "New Blog Post",
      content: "This is the content of the blog post...",
      publishedDate: "2023-08-25T15:00:00Z",
      author: {
        username: "hammy",
        email: "hammy@catppuccin.com",
      },
      tags: ["Technology", "Programming"],
    },
    {
      title: "Another Blog Post",
      content: "This is the content of the blog post...",
      publishedDate: "2023-08-25T15:00:00Z",
      author: {
        username: "pigeon",
        email: "pigeon@catppuccin.com",
      },
      tags: ["Technology", "Programming"],
    },
  ]);
});

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
