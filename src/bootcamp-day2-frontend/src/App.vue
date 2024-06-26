<script setup>
import { ref } from 'vue';
import { bootcamp_day2_backend } from 'declarations/bootcamp-day2-backend/index';
import Blog from "./components/Blog.vue";
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  const num = target.querySelector('#num').value;
  await bootcamp_day2_backend.greet(name, Number(num)).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main class="p-5">
    <br />
    <br />
    <Blog />
  </main>
</template>
