<template>
    <div>
        <div class="flex">
            <h1 class="text-center text-5xl p-5 flex-grow">Mój Blog</h1>
            <button @click="pobierzWpisy" class="bg-blue-900 py-1 px-5 rounded-md text-white m-5">Refresh</button>
        </div>
        <div v-for="wpis in wpisy" class="py-5">
            <div class="bg-gray-600 p-5 my-2 rounded-2xl">
                <p class="text-sm text-gray-400" :style="isDisabled(wpis) ? 'display: block' : 'display: none'">Uploading...</p>
                <h1 class="text-2xl">{{ wpis.name }}</h1>
                <p class="text-wrap text-sm" :style="wpis.id == editing ? `display: none` : `display: block`">{{ wpis.content }}</p>
                <textarea class="p-2" id="" :style="wpis.id == editing ? `display: block` : `display: none`" v-model="edit_content"></textarea>
                <div class="flex justify-center">
                    <button class="bg-gray-800 py-1 px-5 mr-5 rounded-md text-sm disabled:text-gray-400 disabled:bg-gray-700" @click="edytujWpis(wpis.id, wpis.content)" :style="buttonVisibility(wpis)"
                        :disabled="isDisabled(wpis)">Edytuj</button>
                    <button class="bg-gray-800 py-1 px-5 mr-5 rounded-md text-sm disabled:text-gray-400 disabled:bg-gray-700" @click="usunWpis(wpis.id)" :style="buttonVisibility(wpis)"
                        :disabled="isDisabled(wpis)">Usuń
                        wpis</button>
                    <button class="bg-gray-800 py-1 px-5 mr-5 rounded-md text-sm" @click="submitEdit()" :style="buttonVisibility(wpis, true)">Zakończ
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
    editing: string | null,
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

            this.wpisy.push({ name, content, id: "" });
            this.content = "";

            await bootcamp_day2_backend.dodaj_wpis(name, content);
            await this.pobierzWpisy()
        },
        async usunWpis(id: string) {
            this.wpisy = this.wpisy.filter(wpis => wpis.id != id);
            await bootcamp_day2_backend.usun_wpis(id);
            await this.pobierzWpisy();
        },
        edytujWpis(id: string, wpis: string) {
            this.editing = id;
            this.edit_content = wpis;
        },
        async submitEdit() {
            const content = this.edit_content;
            const id = this.editing!;
            const wpis_index = this.wpisy.findIndex((wpis) => wpis.id == id);

            this.wpisy[wpis_index].content = content;

            this.editing = null;
            this.edit_content = "";
            await bootcamp_day2_backend.edit_wpis(id, content)
            this.pobierzWpisy();
        },
        isDisabled(wpis: Wpis) {
            return wpis.id == ""
        },
        buttonVisibility(wpis: Wpis, inverse: boolean = false) {
            const shouldBeHidden = inverse ? wpis.id != this.editing : wpis.id == this.editing;

            return shouldBeHidden ? "display: none" : "display: block";
        }
    },
    mounted() {
        this.pobierzWpisy().then(() => console.log("Fetched blog entries"))
    }
}
</script>