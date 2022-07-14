function steam_export() {  
	const video_card_description = document.getElementById('cat3_details');
	
	let percentages = Array.from(video_card_description.getElementsByClassName('stats_col_right'))
	percentages = percentages.map(p => parseFloat(p.textContent.replace('%', '')));
	
	const names = Array.from(video_card_description.getElementsByClassName('stats_col_mid'));
	
	const data = {};
	
	const products = names.map((element, index) => {
		const share = percentages[index];
		const name = element.textContent;
		
		const vendor_icon = Array.from(element.getElementsByClassName('hws_vendor'))[0];
		if (!vendor_icon) {
			return null;
		}
		
		const vendors = ['nvidia', 'amd', 'intel'];
		
		let vendor = '';
		for (const v of vendors) {
			if (vendor_icon.src.includes(v)) {
				vendor = v;
			}
		}
		
		const for_vendor = data[vendor] || {};
		data[vendor] = for_vendor;
		
		for_vendor[name] = share;
		
		return {name, share, vendor};
	}).filter(e => e);

	console.log(products);
	
	return data;
}

console.log(steam_export())
