<template>
    <div>
        siema blog!
        <div v-for="wpis in wpisy">
            <h5>{{ wpis.name }}</h5>
            <p>{{ wpis.content }}</p>
        </div>
        <div>
            <h1>Dodaj wpis:</h1>
            <div>
                <label for="name">Nazwa użytkownika:</label>
                <input type="text" v-model="name" id="name">
            </div>
            <div>
                <label for="content">Treść:</label>
            </div>
            <div>
                <textarea name="content" id="content" v-model="content"></textarea>
            </div>
            <button @click="dodajWpis">Wyślij</button>
        </div>
    </div>

    <button @click="pobierzWpisy">Refresh</button>
</template>

<script>
import { bootcamp_day2_backend } from 'declarations/bootcamp-day2-backend/index';

export default {
    data() {
        return {
            wpisy: [],
            name: "",
            content: ""
        }
    },
    methods: {
        async pobierzWpisy() {
            this.wpisy = await bootcamp_day2_backend.odczytaj_wpisy()
        },
        async dodajWpis() {
            await bootcamp_day2_backend.dodaj_wpis(this.name, this.content);
            await this.pobierzWpisy()
        }
    },
    mounted() {
        this.pobierzWpisy().then(() => console.log("Fetched blog entries"))
    }
}
</script>