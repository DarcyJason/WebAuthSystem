<script>
    import * as NavigationMenu from "$lib/components/ui/navigation-menu/index";
    import {Button} from "$lib/components/ui/button/index";
    import ToggleTheme from "$lib/components/toggle-theme.svelte";
    import {Separator} from "$lib/components/ui/separator/index";
</script>

<div class="flex flex-col w-screen h-screen">
    <nav class="grid grid-cols-3 h-fit">
        <a class="flex justify-center items-center" href="/">
            <img alt="logo" height="64px" src="/logo.svg" width="64px"/>
            <span class="text-2xl">Homeryland</span>
        </a>
        <div class="justify-self-center self-center">
            <NavigationMenu.Root class="grid grid-cols-6 ">
                <NavigationMenu.List>
                    <NavigationMenu.Item>
                        <NavigationMenu.Trigger>
                            Products
                        </NavigationMenu.Trigger>
                        <NavigationMenu.Content>
                            <ul>
                                <li>
                                    <NavigationMenu.Link href="/">
                                        MyNotes
                                    </NavigationMenu.Link>
                                </li>
                                <li>
                                    <NavigationMenu.Link href="/">
                                        MyDB
                                    </NavigationMenu.Link>
                                </li>
                                <li>
                                    <NavigationMenu.Link href="/">
                                        MyVault
                                    </NavigationMenu.Link>
                                </li>
                            </ul>
                        </NavigationMenu.Content>
                    </NavigationMenu.Item>
                </NavigationMenu.List>
                <NavigationMenu.List>
                    <NavigationMenu.Item>
                        <NavigationMenu.Link href="/">Docs</NavigationMenu.Link>
                    </NavigationMenu.Item>
                </NavigationMenu.List>
                <NavigationMenu.List>
                    <NavigationMenu.Item>
                        <NavigationMenu.Link href="/">
                            Pricing
                        </NavigationMenu.Link>
                    </NavigationMenu.Item>
                </NavigationMenu.List>
                <NavigationMenu.List>
                    <NavigationMenu.Item>
                        <NavigationMenu.Link href="/">
                            Solutions
                        </NavigationMenu.Link>
                    </NavigationMenu.Item>
                </NavigationMenu.List>
                <NavigationMenu.List>
                    <NavigationMenu.Item>
                        <NavigationMenu.Link href="/">
                            Support
                        </NavigationMenu.Link>
                    </NavigationMenu.Item>
                </NavigationMenu.List>
                <NavigationMenu.List>
                    <NavigationMenu.Item>
                        <NavigationMenu.Link href="/">Blog</NavigationMenu.Link>
                    </NavigationMenu.Item>
                </NavigationMenu.List>
            </NavigationMenu.Root>
        </div>
        <div class="flex justify-center items-center">
            <ToggleTheme/>
            <Button variant="ghost">
                <a href="/login">Login in</a>
            </Button>
            <Button>
                <a href="/register">Sign up</a>
            </Button>
        </div>
    </nav>
    <main class="flex flex-1 flex-col w-full justify-center items-center">
        <h1 class="text-4xl">Welcome to Homeryland</h1>
        <p class="text-muted-foreground mt-2">
            Make use of everything.
        </p>
    </main>
    <footer class="w-full px-12 py-8">
        <Separator class="mb-8"/>
        <div class="grid grid-cols-4 gap-8 text-sm text-muted-foreground mb-8">
            <div>
                <div class="flex items-center gap-2 mb-3">
                    <img alt="logo" height="64px" src="/logo.svg" width="64px"/>
                    <span class="text-foreground font-semibold">Homeryland</span
                    >
                </div>
                <p>All you need in here.</p>
            </div>
            <div>
                <p class="text-foreground font-medium mb-3">Products</p>
                <ul class="space-y-2">
                    <li>
                        <a
                                class="hover:text-foreground transition-colors"
                                href="/">MyNotes</a
                        >
                    </li>
                    <li>
                        <a
                                class="hover:text-foreground transition-colors"
                                href="/">MyDB</a
                        >
                    </li>
                    <li>
                        <a
                                class="hover:text-foreground transition-colors"
                                href="/">MyVault</a
                        >
                    </li>
                </ul>
            </div>
            <div>
                <p class="text-foreground font-medium mb-3">Resources</p>
                <ul class="space-y-2">
                    <li>
                        <a
                                class="hover:text-foreground transition-colors"
                                href="/">Docs</a
                        >
                    </li>
                    <li>
                        <a
                                class="hover:text-foreground transition-colors"
                                href="/">Blog</a
                        >
                    </li>
                    <li>
                        <a
                                class="hover:text-foreground transition-colors"
                                href="/">Support</a
                        >
                    </li>
                </ul>
            </div>
            <div>
                <p class="text-foreground font-medium mb-3">Company</p>
                <ul class="space-y-2">
                    <li>
                        <a
                                class="hover:text-foreground transition-colors"
                                href="/">Pricing</a
                        >
                    </li>
                    <li>
                        <a
                                class="hover:text-foreground transition-colors"
                                href="/">Solutions</a
                        >
                    </li>
                </ul>
            </div>
        </div>
        <Separator class="mb-4"/>
        <p class="text-xs text-muted-foreground text-center">
            © {new Date().getFullYear()} Homeryland. All rights reserved.
        </p>
    </footer>
</div>
