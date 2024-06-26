<template>
    <div>
        <div class="flex">
            <h1 class="text-center text-5xl p-5 flex-grow">Mój Blog</h1>
            <button @click="pobierzWpisy" class="bg-blue-900 py-1 px-5 rounded-md text-white m-5">Refresh</button>
        </div>
        <div v-for="(wpis, index) in wpisy" class="py-5">
            <div class="bg-gray-600 p-5 my-2 rounded-2xl">
                <h1 class="text-2xl">{{ wpis.name }}</h1>
                <p class="text-wrap text-sm" :id="`blog_content${index}`" :style="index == editing ? `display: none` : `display: block`">{{ wpis.content }}</p>
                <textarea class="p-2" id="" :style="index == editing ? `display: block` : `display: none`" v-model="edit_content"></textarea>
                <div class="flex justify-center">
                    <button class="bg-gray-800 py-1 px-5 mr-5 rounded-md text-sm" @click="edytujWpis(index, wpis.content)"
                        :style="index == editing ? `display: none` : `display: inline-block`">Edytuj</button>
                    <button class="bg-gray-800 py-1 px-5 mr-5 rounded-md text-sm" @click="usunWpis(index)" :style="index == editing ? `display: none` : `display: inline-block`">Usuń wpis</button>
                    <button class="bg-gray-800 py-1 px-5 mr-5 rounded-md text-sm" @click="submitEdit()" :style="index == editing ? `display: inline-block` : `display: none`">Zakończ
                        edytowanie</button>
                </div>
            </div>
        </div>
        <div class="border-gray-500 border-solid border-2 p-5 rounded-lg">
            <fieldset>
                <legend class="text-3xl">Dodaj wpis:</legend>
                <div class="py-2">
                    <div class="py-5">
                        <label for="name" class="mr-5">Nazwa użytkownika:</label>
                        <input type="text" v-model="name" id="name">
                    </div>
                    <div>
                        <textarea name="content" class="w-full" v-model="content"></textarea>
                    </div>
                </div>
                <button class="bg-blue-900 py-1 px-5 rounded-md text-white m-" @click="dodajWpis">Wyślij</button>
            </fieldset>
        </div>
    </div>

    <div class="grid place-items-center p-5">
    </div>
</template>

<script lang="ts">
import type { Wpis } from '../../../declarations/bootcamp-day2-backend/bootcamp-day2-backend.did';
import { bootcamp_day2_backend } from '../../../declarations/bootcamp-day2-backend/index';

interface Data {
    wpisy: Wpis[],
    name: string,
    content: string,
    editing: number | null,
    edit_content: string
}

export default {
    data(): Data {
        return {
            wpisy: [],
            name: "",
            content: "",
            editing: null,
            edit_content: ""
        }
    },
    methods: {
        async pobierzWpisy() {
            this.wpisy = await bootcamp_day2_backend.odczytaj_wpisy()
        },
        async dodajWpis() {
            const content = this.content;
            const name = this.name;

            this.wpisy.push({ name, content });
            this.content = "";

            await bootcamp_day2_backend.dodaj_wpis(name, content);
            await this.pobierzWpisy()
        },
        async usunWpis(id: number) {
            this.wpisy = this.wpisy.filter((_, index) => index != id);
            await bootcamp_day2_backend.usun_wpis(BigInt(id));
            await this.pobierzWpisy();
        },
        edytujWpis(id: number, wpis: string) {
            this.editing = id;
            this.edit_content = wpis;
        },
        async submitEdit() {
            const content = this.edit_content;
            const id = this.editing!;
            this.wpisy[id].content = content;
            this.editing = null;
            this.edit_content = "";
            await bootcamp_day2_backend.edit_wpis(BigInt(id), content)
            this.pobierzWpisy();
        }
    },
    mounted() {
        this.pobierzWpisy().then(() => console.log("Fetched blog entries"))
    }
}
</script>