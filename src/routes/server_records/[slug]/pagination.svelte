<script>
	export let server_endpoint = '/to_server/';
	export let photo_status = '';
	import { getToServerSettings } from '$lib/rust_funtions';
	import { onMount, createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
	let active_num = 1;
	const change_act_num = (new_num) => {
		active_num = new_num;
	};
	let links = [];
	let MAX_PER_PAGE = 1000;
	const getTranslatedEndpoint = (photo_status) => {
		if (photo_status === 'REV_ACC') {
			return 'accepted';
		}
		if (photo_status === 'REV_REJECT') {
			return 'rejected';
		}
		if (photo_status === 'NOT_REV') {
			return 'not_reviewed';
		}
	};
	const constructLinks = (max_num_rows, photo_status) => {
		let total_rows = 0;
		let server_port = `${to_server.server}`; //:${to_server.port}`;
		let source_url = `${server_port}/count/${getTranslatedEndpoint(photo_status)}`;
		fetch(source_url, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
			.then((resp) => resp.json())
			.then((data) => {
				total_rows = data.count;
				let total_links = total_rows / max_num_rows;
				let offset = 1;
				let base_url = server_port + server_endpoint;
				let created_links = [];
				for (let i = 0; i < total_links; i++) {
					const link = {
						full_path: `${base_url}/${offset}/${max_num_rows}`,
						order: i + 1
					};
					created_links.push(link);
					console.log('generate link:' + link.full_path);
					offset += max_num_rows;
				}
				links = created_links;
			});
	};

	let to_server = {};
	getToServerSettings().then((settings) => {
		to_server.server = settings[0];
		to_server.port = settings[1];
		to_server.user_id = settings[2];
		constructLinks(MAX_PER_PAGE, photo_status);
		let server_port = `${to_server.server}`;
		let base_url = server_port + server_endpoint;
		requestFetch(base_url + '/1/' + MAX_PER_PAGE);
	});
	onMount(() => {
		console.log('Pagination mounted!');
	});
	const requestFetch = (link) => {
		console.log('Will request link:', link);
		dispatch('page_select', {
			req_link: link
		});
	};
	const get_class = (order) => {
		if (active_num === order) {
			return 'flex items-center justify-center border h-full w-full bg-slate-400';
		} else {
			return 'flex items-center justify-center border h-full w-full';
		}
	};
</script>

<ul class="flex items-center justify-center text-black w-full">
	{#each links as link, idx ('server_li_' + photo_status + '-' + idx)}
		<li class="w-7 h-7 flex">
			<button
				on:click={() => {
					requestFetch(link.full_path);
					change_act_num(link.order);
				}}
				class={get_class(link.order)}>{link.order}</button
			>
		</li>
	{/each}
</ul>
