import React from 'react';

const proto = {};
proto.page = require('./page_grpc_web_pb.js');
const client = new proto.page.PageClient('http://127.0.0.1:8000', null, null);

class Page extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
    }

    componentDidMount() {
        const request = new proto.page.PageRequest();
        request.setName(this.props.page);

        client.getPage(request, {}, (err, response) => {
            this.setState({page:
                response.getBlocksList().map((block, index) => {
                    switch (block.getBlockCase()) {
                        case proto.page.Block.BlockCase.PARAGRAPHBLOCK:
                            return <p dangerouslySetInnerHTML={{ __html: block.getParagraphblock().getData().getText() }} />;
                        case proto.page.Block.BlockCase.HEADERBLOCK:
                            const HeadingTag = `h${block.getHeaderblock().getData().getLevel()}`;
                            return <HeadingTag dangerouslySetInnerHTML={{ __html: block.getHeaderblock().getData().getText() }} />;
                        case proto.page.Block.BlockCase.LISTBLOCK:
                            let ListTag;
                            let ItemTag;
                            if (block.getListblock().getData().getStyle() === proto.page.ListStyle.ORDERED) {
                                ListTag = `ol`;
                                ItemTag = 'li';
                            } else {
                                ListTag = `ul`;
                                ItemTag = 'li';
                            }
                            return (
                                <ListTag>
                                    {block.getListblock().getData().getItemsList().map((item, index) =>
                                        <ItemTag dangerouslySetInnerHTML={{ __html: item }} />
                                    )}
                                </ListTag>
                            );
                        default:
                            return <p>Block is not supported</p>;
                    }
                })
            })
        });
    }
    
    render() {return <main>{this.state.page}</main>;}
}
  
export default Page;