import { getChildrenUnsynced, updateSynced } from '$lib/rust_funtions';
export const yearFromPath = (path) => {
	// Extract the filename from the path
	var filename = path.split('\\').pop();

	// Extract the date from the filename
	var date = filename.split('_')[2];

	// Extract the year from the date
	var year = date.split('-')[0];

	return year;
};

export const syncTenRows = (to_server) => {
	getChildrenUnsynced().then((children_rows) => {
		if (children_rows.length > 0) {
			children_rows.forEach((up_child) => {
				const new_ch = {
					child_id: Number(up_child.child_id),
					photo_status: up_child.photo_status,
					rev_notes: up_child.review_notes,
					rev_datetime: up_child.rev_datetime ? up_child.rev_datetime : ''
				};
				const server_path = `${to_server.server}/user/record`;
				console.log('Send post to:', server_path);
				fetch(server_path, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify(new_ch)
				})
					.then((response) => response.json())
					.then((data) => {
						console.log('Success:', data);
						if (data[1] === 1) {
							updateSynced(data[0], data[2]);
						}
					})
					.catch((error) => {
						console.error('Error:', error);
						console.log(JSON.stringify(new_ch));
					});
				//updated_children.push(new_ch);
			});
		}
	});
};
