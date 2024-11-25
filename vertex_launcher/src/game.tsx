interface Game {
    id: number;
    title: string;
    subtitle: string;
    description: string;
    background_image: {
        url: string;
        name: string;
        local_path: string;
        revision: number;
    };
    navigation_icon: {
        url: string;
        name: string;
        local_path: string;
        revision: number;
    };
    download_link: {
        url: string;
        name: string;
        local_path: string;
        revision: number;
    };
    version: string;
    platform: string[];
    tags: string[];
}

