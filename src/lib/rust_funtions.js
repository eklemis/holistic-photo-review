import { invoke } from '@tauri-apps/api/tauri';

export const getPhoto = async (src_path, deg) => {
	//console.log('get photo executed!');
	return await invoke('get_photo_src', { srcPath: src_path, deg: deg });
};

export const getFileList = async (path) => {
	return await invoke('get_file_list', { folderPath: path });
};

export const getPhotoPaths = async (child_id, path) => {
	return await invoke('get_photo_paths', { childId: child_id, folderPath: path });
};

export const retrieveRecords = async (server_path, user_id) => {
	console.log(`calling retrive API for user ${user_id}!`);
	return await invoke('retrieve_user_records', {
		apiPath: server_path + '/user/records/' + user_id
	});
};

export const getChild = async (child_id) => {
	return await invoke('get_child', { childId: Number(child_id) });
};

export const getChildIds = async () => {
	return await invoke('get_child_ids', {});
};

export const getSelectedId = async () => {
	return await invoke('get_selected_id', {});
};

export const updateAccept = async (child_id) => {
	return await invoke('update_accept', { childId: Number(child_id) });
};
export const updateReject = async (child_id, rev_notes) => {
	return await invoke('update_reject', { childId: Number(child_id), revNotes: rev_notes });
};

export const getDbChildCount = async () => {
	return await invoke('get_child_count', {});
};

export const getDbChildCountPsFilter = async (photo_status) => {
	return await invoke('get_child_count_psfilter', { photoStatus: photo_status });
};

export const getToServerSettings = async () => {
	return await invoke('get_to_server', {});
};
export const saveToServerSettings = async (server, port, user_id) => {
	return await invoke('save_to_server', {
		server: server,
		port: Number(port),
		userId: Number(user_id)
	});
};

export const getFilterPsChildren = async (photo_status) => {
	return await invoke('get_children_psfilter', { photoStatus: photo_status });
};

export const getChildrenUnsynced = async () => {
	return await invoke('get_children_unsynced', {});
};

export const updateSynced = async (child_id, dt_synced) => {
	return await invoke('update_synced', { childId: Number(child_id), dtSynced: dt_synced });
};

export const getCountSynced = async () => {
	return await invoke('count_synced', {});
};
export const decryptText = async (text) => {
	return await invoke('decrypt', { txt: text });
};
