<script>
	import { getFilterPsChildren } from '$lib/rust_funtions';
	import Loading from '../../loading.svelte';
	/** @type {import('./$types').PageData} */
	export let data;

	console.log('data:', data);
	let children = [];
	let is_loading = false;
	const retriveList = (photo_status) => {
		is_loading = true;
		getFilterPsChildren(photo_status)
			.then((result) => {
				children = result;
				is_loading = false;
			})
			.catch((err) => {
				console.log('failed get children list');
				is_loading = false;
			});
	};
	retriveList(data.photo_status);
	let opt =
		data.photo_status === 'REV_ACC'
			? 1
			: data.photo_status === 'REV_REJECT'
			? 2
			: data.photo_status === 'NOT_REV'
			? 3
			: 0;
	console.log('opt:', opt);
	const setOpt = (new_opt) => {
		opt = new_opt;
	};
	const menu_base = 'flex items-center px-2 border h-full rounded-sm';
	const menu_more_style = {
		normal: menu_base + '',
		selected: menu_base + ' bg-slate-600 font-bold text-white'
	};
</script>

<div class="border w-full h-full text-[12px]">
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
						href="/user_records/acc"
						class={opt === 1 ? menu_more_style.selected : menu_more_style.normal}
						on:click={() => {
							setOpt(1);
							retriveList('REV_ACC');
						}}>Accepted list</a
					>
				</li>

				<li class="h-full">
					<a
						href="/user_records/rej"
						class={opt === 2 ? menu_more_style.selected : menu_more_style.normal}
						on:click={() => {
							setOpt(2);
							retriveList('REV_REJECT');
						}}>Reported list</a
					>
				</li>
				<li class="h-full">
					<a
						href="/user_records/rem"
						class={opt === 3 ? menu_more_style.selected : menu_more_style.normal}
						on:click={() => {
							setOpt(3);
							retriveList('NOT_REV');
						}}>Remaining list</a
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
		<div class="max-h-[calc(100vh-130px)] overflow-y-scroll max-w-[calc(100vw-14px)]">
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
										<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
											>{child.sex}</td
										>
										<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
											>{child.dob}</td
										>
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
	</div>
</div>
