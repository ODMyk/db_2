<script lang='ts'>
    import * as Card from '$lib/components/ui/card';
    import * as Table from '$lib/components/ui/table';
    import { ScrollArea } from '$lib/components/ui/scroll-area';
	import UserRow from '$lib/components/ui/user-row/UserRow.svelte';
</script>

<Card.Root class="h-[75vh] bg-backgroundSecondary">
    <Card.Content class="space-y-2">
        <ScrollArea class="h-[75vh] w-full" type="scroll" hideDelay={0}>
            <Table.Root class='w-full border-spacing-0'>
                <Table.Header>
                    <Table.Row class="hover:bg-backgroundSecondary">
                        <Table.Head class="w-[120px]">Id</Table.Head>
                        <Table.Head class="w-[248px]">Nickname</Table.Head>
                        <Table.Head class="w-[280px]">Email</Table.Head>
                        <Table.Head class="w-[280px]">Password</Table.Head>
                        <Table.Head class="w-[32px]"></Table.Head>
                    </Table.Row>
                </Table.Header>
                <Table.Body>
                    <UserRow
                        Id={1}
                        Nickname="DMykO"
                        Email="ostapenko.dmytro05@gmail.com"
                        Password="LAKDJBbgvksdga3#SDGK"
                    />

                    <UserRow
                        Id={2}
                        Nickname="VlaPavlen"
                        Email="vladyslav.gay@gmail.com"
                        Password="frontend>>>"
                    />
                </Table.Body>
            </Table.Root>
        </ScrollArea>
    </Card.Content>
</Card.Root>