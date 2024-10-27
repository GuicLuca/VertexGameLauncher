class Game {
    title: string;
    subtitle: string | null;
    description: string | null;
    background_image: string;
    navigation_icon: string;
    download_link: string;
    version: string;
    os: {
        windows: boolean;
        mac: boolean;
        linux: boolean;
        android: boolean;
    };
    tags: string[];

    constructor(title: string, subtitle: string|null, description: string|null, background_image: string, navigation_icon: string, download_link: string, version: string, os: { windows: boolean, mac: boolean, linux: boolean, android: boolean }, tags: string[]) {
        this.title = title;
        this.subtitle = subtitle;
        this.description = description;
        this.background_image = background_image;
        this.navigation_icon = navigation_icon;
        this.download_link = download_link;
        this.version = version;
        this.os = os;
        this.tags = tags;
    }
}