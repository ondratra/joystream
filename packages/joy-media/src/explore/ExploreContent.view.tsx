import { MediaView } from '../MediaView';
import { ExploreContentProps, ExploreContent } from './ExploreContent';

export const ExploreContentView = MediaView<ExploreContentProps>({
  component: ExploreContent,
  resolveProps: async (props) => {
    const { transport } = props;
    const latestVideoChannels = await transport.latestPublicVideoChannels()
    const latestVideos = await transport.latestPublicVideos()
    const featuredVideos = await transport.featuredVideos()
    
    return { featuredVideos, latestVideos, latestVideoChannels };
  }
});

export default ExploreContentView;