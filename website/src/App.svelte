<script>
	import { debug } from "svelte/internal";

	let types = { note: [("text", "largeText")] };

	let items = [];
	let websocketStatus = "initial";
	let textToAdd;
	let textSearch;
	let uiMode = "view";
	let opItem;
	let dispConfig = { order: "newest" };
	let provText = "";

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
		<table>
			<th>id</th>
			<th>time created</th>
			<th>text</th>
			<th>edit</th>

			{#each items as item}
				<tr>
					<td>{item.id}</td>
					<td>{item.time_created}</td>
					<td>{item.text}</td>
					<td
						><button
							on:click={() => {
								opItem = item.id;
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

	{#if uiMode == "create"}
		<label for="text">new note text:</label>
		<input name="text" type="text" bind:value={textToAdd} />

		<button
			on:click={() => {
				websocket.send(
					JSON.stringify({
						create: { item: { note: { text: textToAdd } } },
					})
				);
				textToAdd = "";
				uiMode = "view";
			}}>add</button
		>
	{/if}

	{#if uiMode == "update"}
		{(provText = items.find((item) => {
			return item.id == opItem;
		}).text)}
		<p>
			{items.find((item) => {
				return item.id == opItem;
			}).text}
		</p>
	{/if}
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
