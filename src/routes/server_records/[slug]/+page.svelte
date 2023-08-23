<script>
	import Loading from '../../loading.svelte';
	/** @type {import('./$types').PageData} */
	export let data;
	import Pagination from './pagination.svelte';
	import { getToServerSettings, decryptText } from '$lib/rust_funtions';
	import { exportToExcel } from '$lib/export_excel';
	//import { MagicCrypt } from 'magiccrypt';

	let children = [];
	let is_loading = false;
	let loading_user = false;
	let opt =
		data.photo_status === 'REV_ACC'
			? 1
			: data.photo_status === 'REV_REJECT'
			? 2
			: data.photo_status === 'NOT_REV'
			? 3
			: 0;

	const setOpt = (new_opt) => {
		opt = new_opt;
	};
	const menu_base = 'flex items-center px-2 border h-full rounded-sm';
	const menu_more_style = {
		normal: menu_base + '',
		selected: menu_base + ' bg-slate-600 font-bold text-white'
	};
	let users = [];

	const retrieveUsers = (api_route) => {
		loading_user = true;
		fetch(api_route, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
			.then((resp) => resp.json())
			.then((data) => {
				loading_user = false;
				users = data;
				//console.log('USERS:', users);
			})
			.catch((er) => {
				loading_user = false;
				console.log('Failed retrieve users:', er);
			});
	};
	let to_server = {};
	getToServerSettings().then((settings) => {
		to_server.server = settings[0];
		to_server.port = settings[1];
		to_server.user_id = settings[2];
		let server_port = `${to_server.server}`; //:${to_server.port}`;
		let end_point = server_port + '/users';
		retrieveUsers(end_point);
	});
	const retriveList = (event) => {
		console.log('Pagination click event FIRED!');
		console.log('Ev:', event.detail.req_link);
		is_loading = true;
		fetch(event.detail.req_link, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		})
			.then((resp) => resp.json())
			.then((data) => {
				is_loading = false;
				children = [];
				let n_children = [];
				data.forEach((row, idx) => {
					decryptText(row.community).then((comm) => {
						decryptText(row.school).then((school) => {
							decryptText(row.child_name).then((name) => {
								let new_row = { ...row };
								new_row['reviewer'] = get_reviewer_name(row.user_id);
								new_row['community'] = comm;
								new_row['school'] = school;
								new_row['child_name'] = name;
								n_children.push(new_row);
								if (idx + 1 === data.length) {
									console.log('Children:', n_children);
									children = n_children;
								}
							});
						});
					});
				});
			})
			.catch((er) => {
				is_loading = false;
				console.log('retrieve data failed:', er);
			});
	};

	const get_reviewer_name = (user_id) => {
		if (loading_user) {
			return 'Loading...';
		}
		let reviewer = user_id;
		users.forEach((user) => {
			if (user.id === user_id) {
				reviewer = user.name;
				return;
			}
		});
		return reviewer;
	};
</script>

<div class="w-full max-w-[calc(100vw-24px)] h-full max-h-screen border">
	<div class="w-full h-10 border p-1 rounded-sm bg-slate-200">
		<a
			href="/"
			class="flex items-center justify-center rounded w-20 h-full border text-white font-bold bg-sky-500"
			>Back</a
		>
	</div>
	<div>
		<div class="flex items-center justify-between border">
			<ul class="flex gap-x-1 h-9 items-center p-1">
				<li class="h-full">
					<a
						href="/server_records/acc"
						class={opt === 1 ? menu_more_style.selected : menu_more_style.normal}
						on:click={() => {
							setOpt(1);
							//retriveList('REV_ACC');
						}}>Accepted list</a
					>
				</li>

				<li class="h-full">
					<a
						href="/server_records/rej"
						class={opt === 2 ? menu_more_style.selected : menu_more_style.normal}
						on:click={() => {
							setOpt(2);
							//retriveList('REV_REJECT');
						}}>Reported list</a
					>
				</li>
				<li class="h-full">
					<a
						href="/server_records/rem"
						class={opt === 3 ? menu_more_style.selected : menu_more_style.normal}
						on:click={() => {
							setOpt(3);
							//retriveList('NOT_REV');
						}}>Remaining list</a
					>
				</li>
				<li class="border w-[2px] h-full" />
				<li class="h-full">
					<button
						class=" rounded w-44 border h-full text-sm bg-green-600 text-white"
						on:click={() => exportToExcel(children, 'children', data.photo_status)}
						>Export to excel</button
					>
				</li>
			</ul>
			<div class="px-2 flex items-center gap-x-2">
				<span class="text-[12px] font-bold text-gray-700">
					{children.length}
				</span>
				<p>Records</p>
			</div>
		</div>
	</div>
	<div class="max-h-[calc(100vh-150px)] overflow-y-scroll max-w-[calc(100vw-24px)] border">
		<div class="table-wrp block h-full">
			{#if is_loading}
				<Loading thing_name="table data" />
			{:else}
				<table class="w-full">
					<thead class="border border-red-700 sticky -top-1 z-10">
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>No.</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Community</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Child id</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Name</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Sex</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>DOB</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>School</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Last grade</th
						>
						{#if opt === 2}
							<th
								class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
								>Report notes</th
							>
						{/if}
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Reviewer</th
						>
					</thead>
					<tbody class="h-96 overflow-y-auto">
						{#if children.length > 0}
							{#each children as child, idx (data.photo_status + '-' + idx)}
								<tr>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1">{idx + 1}</td>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{child.community}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{child.child_id}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{child.child_name}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1">{child.sex}</td>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1">{child.dob}</td>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{child.school}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{child.last_grade}</td
									>
									{#if opt === 2}
										<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
											>{child.review_notes}</td
										>
									{/if}
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{get_reviewer_name(child.reviewer)}</td
									>
								</tr>
							{/each}
						{:else}
							<p class="flex items-center justify-center w-full text-gray-500">No Records</p>
						{/if}
					</tbody>
				</table>
			{/if}
		</div>
	</div>
	<div class="mt-4">
		{#if data.photo_status === 'REV_ACC'}
			<Pagination
				server_endpoint="/all_accepteds"
				photo_status={data.photo_status}
				on:page_select={retriveList}
			/>
		{:else if data.photo_status === 'REV_REJECT'}
			<Pagination
				server_endpoint="/all_rejecteds"
				photo_status={data.photo_status}
				on:page_select={retriveList}
			/>
		{:else}
			<Pagination
				server_endpoint="/all_remainings"
				photo_status={data.photo_status}
				on:page_select={retriveList}
			/>
		{/if}
	</div>
</div>
