use acadrust::{CadDocument, EntityType};

use crate::command::EntityTransform;
use crate::scene::acad_to_truck::TruckEntity;
use crate::scene::object::{GripApply, GripDef, PropSection};
use crate::scene::tess_util::FallbackGeometry;

pub trait TruckConvertible {
    fn to_truck(&self, document: &CadDocument) -> Option<TruckEntity>;
}

/// Fallback geometry for entities not routed through the truck topology
/// pipeline (Viewport, Insert, Hatch outline, Ole2Frame). Returns
/// world-offset-relative `f32` points + snap/key vertices the
/// dispatcher wraps into a `WireModel`.
pub trait FallbackTess {
    fn fallback_geometry(&self, world_offset: [f64; 3]) -> FallbackGeometry;
}

pub trait Grippable {
    fn grips(&self) -> Vec<GripDef>;
    fn apply_grip(&mut self, grip_id: usize, apply: GripApply);
}

pub trait PropertyEditable {
    fn geometry_properties(&self, text_style_names: &[String]) -> PropSection;
    fn apply_geom_prop(&mut self, field: &str, value: &str);
}

pub trait Transformable {
    fn apply_transform(&mut self, t: &EntityTransform);
}

pub trait EntityTypeOps {
    fn to_truck_entity(&self, document: &CadDocument) -> Option<TruckEntity>;
    fn grips(&self) -> Vec<GripDef>;
    fn geometry_properties(&self, text_style_names: &[String]) -> Option<PropSection>;
    fn apply_geom_prop(&mut self, field: &str, value: &str);
    fn apply_grip(&mut self, grip_id: usize, apply: GripApply);
    fn apply_transform(&mut self, t: &EntityTransform);
}

/// Per-dispatch-function entity-variant lists. Adding a new entity that
/// participates in a dispatch = adding one identifier to one list here
/// (instead of one full match arm to each of five `match self` blocks).
///
/// `dispatch!` expands `EntityType::$Variant(e) => $body` for each name.
macro_rules! dispatch {
    ($self:expr, |$e:ident| $body:expr, [$($variant:ident),* $(,)?], _ => $default:expr $(,)?) => {
        match $self {
            $(EntityType::$variant($e) => $body,)*
            _ => $default,
        }
    };
}

impl EntityTypeOps for EntityType {
    fn to_truck_entity(&self, document: &CadDocument) -> Option<TruckEntity> {
        dispatch!(self,
            |e| TruckConvertible::to_truck(e, document),
            [
                Point, Line, Circle, Arc, Ellipse, Spline, LwPolyline,
                Polyline, Polyline2D, Polyline3D, Ray, XLine, RasterImage,
                Wipeout, AttributeDefinition, AttributeEntity, MLine,
                Tolerance, Solid, Face3D, PolygonMesh, PolyfaceMesh, Mesh,
                Table, Text, MText, Leader, MultiLeader, Underlay, Shape,
                Ole2Frame,
            ],
            _ => None,
        )
    }

    fn grips(&self) -> Vec<GripDef> {
        dispatch!(self,
            |e| Grippable::grips(e),
            [
                Line, Circle, Arc, Ellipse, LwPolyline, Polyline, Polyline2D,
                Polyline3D, Ray, XLine, RasterImage, Wipeout,
                AttributeDefinition, AttributeEntity, MLine, Tolerance,
                Solid, Solid3D, Region, Body, Face3D, PolygonMesh,
                PolyfaceMesh, Mesh, Table, Point, Spline, Text, MText,
                Viewport, Insert, Leader, MultiLeader, Dimension, Hatch,
                Underlay, Shape, Ole2Frame,
            ],
            _ => vec![],
        )
    }

    fn geometry_properties(&self, text_style_names: &[String]) -> Option<PropSection> {
        dispatch!(self,
            |e| Some(PropertyEditable::geometry_properties(e, text_style_names)),
            [
                Line, Circle, Arc, Ellipse, LwPolyline, Polyline, Polyline2D,
                Polyline3D, Ray, XLine, RasterImage, Wipeout,
                AttributeDefinition, AttributeEntity, MLine, Tolerance,
                Solid, Solid3D, Region, Body, Face3D, PolygonMesh,
                PolyfaceMesh, Mesh, Table, Hatch, Point, Spline, Text, MText,
                Viewport, Insert, Dimension, Leader, MultiLeader, Underlay,
                Shape, Ole2Frame,
            ],
            _ => None,
        )
    }

    fn apply_geom_prop(&mut self, field: &str, value: &str) {
        dispatch!(self,
            |e| PropertyEditable::apply_geom_prop(e, field, value),
            [
                Line, Circle, Arc, Ellipse, LwPolyline, Polyline, Polyline2D,
                Polyline3D, Ray, XLine, RasterImage, Wipeout,
                AttributeDefinition, AttributeEntity, MLine, Tolerance,
                Solid, Solid3D, Region, Body, Face3D, PolygonMesh,
                PolyfaceMesh, Mesh, Table, Hatch, Point, Spline, Text, MText,
                Viewport, Insert, Dimension, Leader, MultiLeader, Underlay,
                Shape, Ole2Frame,
            ],
            _ => {},
        )
    }

    fn apply_grip(&mut self, grip_id: usize, apply: GripApply) {
        dispatch!(self,
            |e| Grippable::apply_grip(e, grip_id, apply),
            [
                Line, Circle, Arc, Ellipse, LwPolyline, Polyline, Polyline2D,
                Polyline3D, Ray, XLine, RasterImage, Wipeout,
                AttributeDefinition, AttributeEntity, MLine, Tolerance,
                Solid, Solid3D, Region, Body, Face3D, PolygonMesh,
                PolyfaceMesh, Mesh, Table, Point, Spline, Text, MText,
                Viewport, Insert, Leader, MultiLeader, Dimension, Hatch,
                Underlay, Shape, Ole2Frame,
            ],
            _ => {},
        )
    }

    fn apply_transform(&mut self, t: &EntityTransform) {
        dispatch!(self,
            |e| Transformable::apply_transform(e, t),
            [
                Arc, Circle, Ellipse, Hatch, Insert, Line, LwPolyline,
                Polyline, Polyline2D, Polyline3D, Ray, XLine, RasterImage,
                Wipeout, AttributeDefinition, AttributeEntity, MLine,
                Tolerance, Solid, Face3D, PolygonMesh, PolyfaceMesh, Mesh,
                Table, MText, Point, Spline, Text, Viewport, Dimension,
                Leader, MultiLeader, Underlay, Shape, Ole2Frame,
            ],
            _ => {},
        )
    }
}
