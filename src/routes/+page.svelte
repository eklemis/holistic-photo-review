<script>
	import { onMount, afterUpdate, onDestroy } from 'svelte';
	import {
		getPhoto,
		getPhotoPaths,
		getChild,
		getSelectedId,
		updateAccept,
		updateReject,
		getDbChildCount,
		getDbChildCountPsFilter,
		getToServerSettings,
		getCountSynced
	} from '$lib/rust_funtions';
	import { photoPath, masterRetrieved } from '$lib/store';
	import { yearFromPath, syncTenRows } from '$lib/general_purposes';
	import BiggerPicture from 'bigger-picture/svelte';
	import 'bigger-picture/css';
	import Loading from './loading.svelte';

	//STORES subscribtion an unsubscribtion
	let source_path = '';
	const uns_photoPath = photoPath.subscribe((value) => {
		source_path = value;
	});
	let master_retrieved;
	let link_dis_style = '';
	$: link_dis_style = master_retrieved ? 'pointer-events: none' : '';
	const uns_masterRetrieved = masterRetrieved.subscribe((value) => {
		master_retrieved = value;
	});
	let link_dis_class = '';
	$: link_dis_class = master_retrieved
		? 'border h-9 bg-slate-50 rounded p-3 text-slate-400 font-bold flex items-center'
		: 'border h-9 bg-blue-500 rounded p-3 text-white font-bold flex items-center';
	let to_server = {};
	let num_synced = 0;
	const syncId = setInterval(() => {
		syncTenRows(to_server);
		setTimeout(() => {
			getCountSynced().then((c) => {
				num_synced = c;
			});
		}, 100);
	}, 5000);
	const unsubAll = () => {
		uns_photoPath();
		uns_masterRetrieved();
		clearInterval(syncId);
	};
	onDestroy(unsubAll);

	//Current image settups
	let curr_img = {
		path: '',
		source: '',
		year: 0
	};
	let do_show_notes = false;
	const hide_notes = () => {
		do_show_notes = false;
	};
	const show_notes = () => {
		do_show_notes = true;
	};

	//Loading states
	let is_loading_photos = false;
	let is_failed_loading = false;

	//progress states
	let chart;
	let progress = {
		all: 0,
		rejected: 0,
		accepted: 0
	};
	const chart_data = {
		labels: ['All'],
		datasets: [
			{
				label: 'Accepted',
				data: [progress.accepted],
				borderColor: 'rgb(75, 192, 192)',
				backgroundColor: 'rgba(75, 192, 192, 0.6)',
				borderWidth: 1,
				borderRadius: 1,
				borderSkipped: false
			},
			{
				label: 'Reported',
				data: [progress.rejected],
				borderColor: 'rgb(255, 205, 86)',
				backgroundColor: 'rgba(255, 205, 86, 0.6)',
				borderWidth: 1,
				borderRadius: 1,
				borderSkipped: false
			},
			{
				label: 'Remaining',
				data: [progress.all - progress.accepted - progress.rejected],
				borderColor: 'rgb(75, 75, 192)',
				backgroundColor: 'rgba(75, 75, 192, 0.6)',
				borderWidth: 1,
				borderRadius: 1,
				borderSkipped: false
			}
		]
	};
	const refreshProgress = () => {
		getDbChildCount().then((c) => {
			progress.all = c;
			getDbChildCountPsFilter('REV_REJECT').then((c2) => {
				progress.rejected = c2;
				getDbChildCountPsFilter('REV_ACC').then((c3) => {
					progress.accepted = c3;
					if (chart) {
						chart.update();
					}
				});
			});
		});
	};

	//GENERAL SETUPS
	let image_list = [];
	let formated_im_list = [];

	let curr_child = {
		child_id: 0,
		child_name: 'Sample Name',
		sex: 'F',
		dob: '5-18-2023',
		last_grade: 'sixth',
		school: 'SMP Negeri 1 Lamboya',
		community: 'Lamboya',
		photo_status: '',
		rev_notes: '',
		rev_datetime: '',
		synced: false
	};
	$: console.log('Formated list:', formated_im_list);

	// initialize bigger picture
	let bp = BiggerPicture({
		target: document.body
	});

	const retrievePhotos = async (child_id) => {
		console.log(`Send request:(${child_id}, ${source_path})`);
		is_loading_photos = true;
		getPhotoPaths(Number(child_id), source_path)
			.then((paths) => {
				const fmt_list = [];
				if (!paths.length || paths.length === 0) {
					is_failed_loading = true;
					return;
				}
				paths.forEach(async (im_path, path_idx) => {
					getPhoto(im_path).then((img_src) => {
						fmt_list.push({
							path: im_path,
							source: img_src,
							year: yearFromPath(im_path)
						});
						//console.log(img_src);
						if (fmt_list.length === paths.length) {
							//sort the fmt list
							fmt_list.sort(function (a, b) {
								return b.year - a.year;
							});
							formated_im_list = fmt_list;
							//console.log('Formatted list:', formated_im_list);
							is_loading_photos = false;
							is_failed_loading = false;
							return;
						}
					});
				});
			})
			.catch((er) => {
				is_failed_loading = true;
				console.log('Loading photo failed! ', err);
			});
	};
	const retrieveChildData = async (child_id) => {
		getChild(child_id).then((child) => {
			curr_child = { ...child };
		});
	};
	//Current image states management
	const get_photo = async (path) => {
		curr_img.source = await getPhoto(path);
	};
	const getCurrentPhoto = async () => {
		await get_photo(image_list[0]);
	};
	const go_next = async () => {
		acceptPhoto(curr_child.child_id);
		getCurrentPhoto();
		retrieveChildData(curr_child.child_id);
	};
	const getNext = () => {
		getSelectedId().then((id) => {
			refreshProgress();
			retrieveChildData(id);
			retrievePhotos(id);
		});
	};
	const acceptPhoto = async (child_id) => {
		console.log('Try Sending Update Accepted for child:', child_id);
		updateAccept(child_id).then((flag) => {
			if (flag > 0) {
				console.log('Succesfully update');
				getNext();
			} else {
				console.log('Update Failed!');
			}
		});
	};
	const rejectPhoto = async (child_id, rev_notes) => {
		console.log('Try Sending Update Accepted for child:', child_id, rev_notes);
		updateReject(child_id, rev_notes).then((flag) => {
			if (flag > 0) {
				console.log('Succesfully update REJECT');
				getNext();
			} else {
				console.log('Update REJECT Failed!');
			}
			hide_notes();
		});
	};

	onMount(async () => {
		getNext();
		getDbChildCount().then((c) => {
			console.log('Num of child:', c);
			if (c !== 0) {
				masterRetrieved.set(true);
			} else {
				masterRetrieved.set(false);
			}
		});
		const imageLinks = document.querySelectorAll('#images > a');
		// add click listener to open BiggerPicture
		for (let link of imageLinks) {
			link.addEventListener('click', openGallery);
		}
		function openGallery(e) {
			e.preventDefault();

			setTimeout(() => {
				bp.open({
					items: imageLinks,
					el: e.currentTarget
				});
			}, 0);
		}
		getToServerSettings()
			.then((setup) => {
				to_server.server = setup[0];
				to_server.port = setup[1];
				to_server.user_id = setup[2];
				console.log('GOT TO SERVER SETUPS:', setup);
			})
			.catch((err) => {
				console.log('GET TO_SERVER SETUPS FAILED:', err);
			});
	});
	afterUpdate(() => {
		console.log('AFTER UPDATE EXECUTED!');
	});
</script>

{#if do_show_notes}
	<div
		class="absolute w-full h-full opacity-[0.98] z-50 bg-slate-800 flex items-center justify-center"
	>
		<div class="w-96 h-96 bg-white opacity-100 rounded p-8">
			<p class=" font-bold">Report Notes</p>
			<div class="flex flex-col gap-y-3 justify-start mt-4">
				<textarea class="border p-2 outline-none" rows="8" bind:value={curr_child.rev_notes} />
				<div>
					<input
						type="button"
						value="Report"
						class="border rounded h-9 w-24 bg-red-600 text-white"
						on:click={() => rejectPhoto(curr_child.child_id, curr_child.rev_notes)}
					/>
					<input
						type="button"
						on:click={() => hide_notes()}
						value="Cancel"
						class="border rounded h-9 w-24 bg-slate-200"
					/>
				</div>
			</div>
		</div>
	</div>
{/if}

<div class="flex flex-col p-4 gap-y-4 h-full w-[360px] min-w-[360px] border relative">
	<div class="flex flex-col items-end justify-end gap-x-1 py-1">
		<!--
		<div class="flex items-center gap-x-5">
			<p>{to_server.server}-></p>
			<p>{to_server.port}-></p>
			<p>{to_server.user_id}-></p>
		</div>
        -->
		<a href="/to_server" alt="open retrieve page" class={link_dis_class} style={link_dis_style}
			>Retrieve master data</a
		>
	</div>
	<div class=" flex justify-between w-full text-[12px] mt-10 border bg-white rounded">
		<input
			type="text"
			name="child_id"
			class="w-full outline-none px-1 rounded-l"
			bind:value={curr_child.child_id}
		/>
		<button
			on:click={() => {
				retrievePhotos(curr_child.child_id);
				retrieveChildData(curr_child.child_id);
			}}
			class="border-l rounded bg-slate-100 h-8 min-w-[120px] flex items-center justify-center"
			>Find child</button
		>
	</div>
	<h3 class="text-[12px] font-bold text-[#423C3C]">Identity</h3>

	<div class="flex gap-x-2 items-baseline justify-between">
		<p class="flex w-24 text-[12px] text-gray-400 text-left">Child id</p>
		<span
			class="focus:outline-none w-16 bg-white border-b border-b-gray-100 text-[#423C3C] text-[12px] font-bold text-right"
			>{curr_child.child_id}</span
		>
	</div>
	<div class="flex gap-x-2 items-baseline justify-between">
		<p class="flex w-24 text-[12px] text-gray-400 text-left">Child name</p>
		<span class="text-[#423C3C] font-bold text-[12px]">{curr_child.child_name}</span>
	</div>
	<div class="flex gap-x-4 items-baseline justify-between">
		<p class="flex w-24 text-[12px] text-gray-400 text-left">DOB</p>

		<div class="flex gap-x-2 items-baseline justify-between">
			<span class="text-[#423C3C] font-bold text-[12px]">{curr_child.dob}</span>
			<div class="flex justify-end gap-x-2">
				<p class="flex text-[12px] text-gray-400 text-left">Sex</p>
				<span class="text-[#423C3C] font-bold text-[12px]">{curr_child.sex}</span>
			</div>
		</div>
	</div>
	<div class="flex gap-x-2 items-baseline justify-between">
		<p class="flex text-[12px] text-gray-400 text-left">School</p>
		<div class="flex gap-x-2 items-baseline justify-between">
			<span class="text-[#423C3C] font-bold text-[12px]">{curr_child.school}</span>
			<div class="flex justify-end gap-x-2">
				<p class="flex text-[12px] text-gray-400 text-left">Grade</p>
				<span class="text-[#423C3C] font-bold text-[12px]"
					>{curr_child.last_grade.replace('grade', '')}</span
				>
			</div>
		</div>
	</div>

	<div class="flex gap-x-2 items-baseline justify-between">
		<p class="flex text-[12px] text-gray-400 text-left">Community</p>
		<span class="text-[#423C3C] font-bold text-[12px]">{curr_child.community}</span>
	</div>

	<div class="flex h-9 w-full gap-x-2 items-center justify-left mt-4">
		<div>
			<button
				class="border h-8 w-20 rounded-md bg-green-600 text-white font-bold text-[12px]"
				on:click={go_next}>Accept</button
			>
		</div>
		<button
			class="border h-8 w-28 rounded-md bg-[#EA4C4C] text-white font-bold text-[12px]"
			on:click={() => {
				show_notes();
			}}>Report Problem</button
		>
	</div>
	<div class="border-t p-4 absolute left-0 bottom-0 w-full text-[12px]">
		<div class="flex items-start justify-between gap-x-2 w-full">
			<div class="w-1/2">
				<p class="font-bold text-[#423C3C]">Your progress</p>
				<div class="w-full pr-2 flex flex-col gap-y-1">
					<div class="flex items-center justify-between">
						<p class="flex text-[12px] text-gray-400 text-left">Accepted</p>
						<a href="/user_records/acc" class="text-green-600 font-bold text-[12px] underline"
							>{progress.accepted}</a
						>
					</div>
					<div class="flex items-center justify-between">
						<p class="flex text-[12px] text-gray-400 text-left">Reported</p>
						<a href="/user_records/rej" class="text-red-600 font-bold text-[12px] underline"
							>{progress.rejected}</a
						>
					</div>
					<div class="flex items-center justify-between">
						<p class="flex text-[12px] text-gray-400 text-left">Remaining</p>
						<a href="/user_records/rem" class="text-[#423C3C] font-bold text-[12px] underline"
							>{progress.all - progress.accepted - progress.rejected}</a
						>
					</div>
				</div>
			</div>
			<div class="w-1/2 flex flex-col h-[100px] items-center">
				<div>
					<p class="font-bold text-gray-400">Review Progress</p>
					<div class="w-full flex items-center justify-center">
						{#if progress.all === progress.accepted + progress.rejected}
							<span class="text-xl text-green-600">100%</span>
						{:else}
							<span class="text-xl">{progress.accepted + progress.rejected}</span>
							<span class="text-xl text-gray-400">/</span>
							<span class="text-xl">{progress.all}</span>
						{/if}
					</div>
				</div>
				<div>
					<p class="font-bold text-gray-400">Sync Progress</p>
					<div class="w-full flex items-center justify-center">
						{#if num_synced === progress.accepted + progress.rejected}
							<span class="text-xl text-green-600">100%</span>
						{:else}
							<span class="text-xl">{num_synced}</span>
							<span class="text-xl text-gray-400">/</span>
							<span class="text-xl">{progress.accepted + progress.rejected}</span>
						{/if}
					</div>
				</div>
			</div>
			<div />
		</div>
		<div class="flex w-full">
			<a
				href="/server_records/acc"
				class="bg-blue-500 h-[32px] w-full text-white font-bold flex rounded items-center justify-center"
				>Holistic Tracking</a
			>
		</div>
	</div>
</div>
<div
	id="images"
	class="flex items-start flex-wrap gap-1 border w-full h-full p-2 overflow-y-scroll"
>
	{#if is_loading_photos && master_retrieved}
		<Loading thing_name="Photos" />
	{:else if is_failed_loading}
		<p class=" text-yellow-500">Failed Loading Photos!</p>
	{:else}
		{#each formated_im_list as im_meta}
			<a
				href={im_meta.source}
				data-img={im_meta.source}
				data-thumb={im_meta.source}
				data-alt={`Child photo of year ${im_meta.year}`}
				data-height="3000"
				data-width="2000"
				class="border flex flex-col justify-center items-center w-[248px] p-1 bg-slate-200 rounded-sm"
			>
				<img
					class="rounded-sm"
					src={im_meta.source}
					alt={`Child photo of year ${im_meta.year}`}
					id="active"
					width="240"
					height="320"
				/>
				<div class="relative flex w-[392px] items-center justify-between px-1 pt-1">
					<div class="flex justify-center w-full">
						<span class="font-bold text-[12px] text-gray-500">{im_meta.year}</span>
					</div>
				</div>
			</a>
		{/each}
	{/if}
</div>

<style lang="postcss">
	:global(html) {
		background-color: theme(colors.gray.100);
	}
</style>
