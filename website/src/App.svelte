<script>
	import { claim_space } from "svelte/internal";

	// let types = { note: [("text", "largeText")] };

	let items = [];
	let websocketStatus = "initial";
	let textSearch;
	let uiMode = "view";
	let opItem;
	let dispConfig = { order: "newest" };
	let provText = "";
	let itemTypes = {
		note: { text: ["text", "note text:"] },
		person: { name: ["text", "name:"], nickname: ["text", "nickname:"] },
	};
	let opType = Object.keys(itemTypes)[0];

	console.log(itemTypes[opType]);

	const websocket = new WebSocket("ws://127.0.0.1:5000/ws");
	websocket.addEventListener("open", () => {
		console.log("websocket connected");
		websocketStatus = "open";
		websocket.send('{"read": {}}');
	});
	websocket.addEventListener("message", ({ data }) => {
		console.log(data);
		items = JSON.parse(data);
		items = items.map((item) => {
			item.time_created = Date.parse(item.time_created);
			return item;
		});

		switch (dispConfig.order) {
			case "newest":
		}
	});
	websocket.addEventListener("close", () => {
		websocketStatus = "closed";
	});
</script>

<main>
	<h1>Sybil</h1>

	{#if websocketStatus == "closed"}
		<h2>Websocket disconnected</h2>
	{/if}

	<input
		type="text"
		name="text_search"
		id="text_search"
		bind:value={textSearch}
	/>

	<button
		on:click={() => {
			// websocket.send('{"read": {}}');
			textSearch = textSearch == "" ? null : textSearch;
			websocket.send(
				JSON.stringify({ read: { text_search: textSearch } })
			);
			uiMode = "view";
		}}>search</button
	>

	<button
		on:click={() => {
			uiMode = "create";
		}}>add</button
	>

	{#if uiMode == "view"}
		<br />
		<select bind:value={opType}>
			{#each Object.keys(itemTypes) as itemType}
				<option value={itemType}>{itemType}</option>
			{/each}
		</select>

		<table>
			<th>id</th>
			<th>time created</th>
			<!-- <th>text</th> -->
			{#each Object.keys(itemTypes[opType]) as type}
				<th>{type}</th>
			{/each}

			<th>edit</th>

			{#each items.filter((i) => i.type == opType) as item}
				<tr>
					<td>{item.id}</td>
					<td>{item.time_created}</td>

					<!-- <td>{item.text}</td> -->
					{#each Object.keys(itemTypes[opType]) as type}
						<td>{item[type]}</td>
					{/each}

					<td
						><button
							on:click={() => {
								opItem = item.id;
								opType = item.type;
								uiMode = "update";
							}}>update</button
						></td
					>
				</tr>
			{:else}
				<p>No Results</p>
			{/each}
		</table>
	{/if}
	<!-- view  -->

	{#if uiMode == "create"}
		<br />
		<select bind:value={opType}>
			{#each Object.keys(itemTypes) as itemType}
				<option value={itemType}>{itemType}</option>
			{/each}
		</select>

		{#each Object.entries(itemTypes[opType]) as field}
			{#if field[1][0] == "text"}
				<label for={field[0]}>{field[1][1]}</label>
				<input type="text" name={field[0]} id={field[0]} />
			{:else}
				<h1>ERROR: UNKNOWN FIELD TYPE</h1>
			{/if}
		{/each}

		<button
			on:click={() => {
				let draftItem = {};

				console.log(itemTypes[opType]);

				let fields = Object.keys(itemTypes[opType]);
				console.log("fields: ", fields);
				for (let i = 0; i < fields.length; i++) {
					console.log(fields[i]);
					let formField = document.getElementById(fields[i]);
					draftItem[fields[i]] = formField.value;
				}

				console.log(draftItem);

				let itemToSend = {};
				itemToSend[opType] = draftItem;

				console.log(JSON.stringify(itemToSend));

				websocket.send(
					JSON.stringify({
						create: { item: itemToSend },
					})
				);
				uiMode = "view";
			}}>add</button
		>
	{/if}
	<!-- create  -->

	{#if uiMode == "update"}
		<!-- <p>
			{items.find((item) => {
				return item.id == opItem;
			}).text}
		</p> -->

		{#each Object.entries(itemTypes[opType]) as field}
			{#if field[1][0] == "text"}
				<label for={field[0]}>{field[1][1]}</label>
				<input
					type="text"
					name={field[0]}
					id={field[0]}
					value={items.find((item) => {
						return item.id == opItem;
					})[field[0]]}
				/>
			{:else}
				<h1>ERROR: UNKNOWN FIELD TYPE</h1>
			{/if}
		{/each}

		<button
			on:click={() => {
				let draftItem = {};

				console.log(itemTypes[opType]);

				let fields = Object.keys(itemTypes[opType]);
				console.log("fields: ", fields);
				for (let i = 0; i < fields.length; i++) {
					console.log(fields[i]);
					let formField = document.getElementById(fields[i]);
					draftItem[fields[i]] = formField.value;
				}

				console.log(draftItem);

				let itemToSend = {};
				itemToSend[opType] = draftItem;

				console.log(JSON.stringify(itemToSend));

				websocket.send(
					JSON.stringify({
						update: { id: opItem, item: itemToSend },
					})
				);
				uiMode = "view";
			}}>update</button
		>
	{/if}
	<!-- update  -->
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	table,
	th,
	td {
		border: 1px solid black;
		border-collapse: collapse;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
