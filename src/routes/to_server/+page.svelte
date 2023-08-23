<script>
	import { retrieveRecords, getToServerSettings, saveToServerSettings } from '$lib/rust_funtions';
	import { toServer, masterRetrieved } from '$lib/store';
	import { onMount } from 'svelte';

	let setups = {
		server: '',
		port: 0,
		user_id: 0
	};

	let req_status = {
		send: false,
		success: false
	};
	const resetStatus = () => {
		req_status = {
			send: false,
			success: false
		};
	};
	const updateToServer = (base) => {
		setups.server = base[0];
		setups.port = base[1];
		setups.user_id = base[2];
		toServer.set(setups);
	};
	onMount(() => {
		getToServerSettings().then((res) => {
			console.log('to server:', res);
			updateToServer(res);
		});
	});
</script>

<div class="border p-4">
	<a
		href="/"
		class="border w-20 h-9 bg-blue-500 rounded p-3 text-white font-bold flex items-center justify-center"
		>Back</a
	>
	<div class="w-full flex flex-col gap-y-4 mt-8 text-[14px]">
		<div class="flex gap-x-4">
			<label class="w-24 text-gray-400" for="server">Server</label>
			<input
				type="text"
				id="server"
				bind:value={setups.server}
				class="border outline-none px-2 w-32"
			/>
			<label for="port" class="text-gray-400">Port</label>
			<input
				id="port"
				type="number"
				bind:value={setups.port}
				class="border outline-none px-2 w-24"
			/>
		</div>
		<div class="flex gap-x-4">
			<label class="w-24 text-gray-400" for="user_id">User Id</label>
			<input
				id="user_id"
				type="number"
				bind:value={setups.user_id}
				class="border outline-none px-2 w-10"
			/>
		</div>
		<button
			on:click={() => {
				req_status.send = true;
				console.log('Send request:', setups.server + ':' + setups.port + '/' + setups.user_id);
				retrieveRecords(setups.server + ':' + setups.port, setups.user_id).then((result) => {
					if (result > 0) {
						req_status.success = true;
						masterRetrieved.update((v) => !v);
					}
					console.log('Result:', result);
					saveToServerSettings(setups.server, setups.port, setups.user_id);
					//updateToServer(setups);
				});
			}}
			class="border h-8 w-32 rounded-md bg-green-600 text-white font-bold mt-2"
			>Retrieve Data</button
		>
	</div>

	{#if req_status.send}
		<div>
			<p>Sending request...</p>
		</div>
	{/if}
	{#if req_status.success}
		<div>
			<p>Data Succesfully Retrieved!</p>
		</div>
	{/if}
</div>
