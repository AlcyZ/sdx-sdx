import {createApp} from 'vue';
import App from './App.vue';

export default async (): Promise<void> => {
	const app = createApp(App);
	
	app.mount('#app');
};
